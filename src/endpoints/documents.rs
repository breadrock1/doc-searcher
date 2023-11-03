use crate::context::SearchContext;
use crate::errors::{SuccessfulResponse, WebError, WebResponse};
use crate::wrappers::Document;

use actix_web::{delete, get, post, put, web, HttpResponse, ResponseError};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::Method;
use elasticsearch::BulkParts;
use serde::Deserialize;
use serde_json::{json, Value};

#[put("/document/update")]
async fn update_document(cxt: web::Data<SearchContext>, form: web::Json<Document>) -> HttpResponse {
    let elastic = cxt.get_cxt().read().await;
    let bucket_name = &form.bucket_uuid;
    let document_id = &form.document_md5_hash;
    let document_ref = &form.0;
    let document_json = deserialize_document(document_ref);
    if document_json.is_err() {
        let err = document_json.err().unwrap();
        let web_err = WebError::UpdateDocument(err.to_string());
        return web_err.error_response();
    }

    let document_json = document_json.unwrap();
    let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
    let response_result = elastic
        .send(
            Method::Put,
            s_path.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(document_json.to_string().as_bytes()),
            None,
        )
        .await;

    match response_result {
        Ok(_) => SuccessfulResponse::ok_response("Ok"),
        Err(err) => {
            let web_err = WebError::UpdateDocument(err.to_string());
            web_err.error_response()
        }
    }
}

#[post("/document/new")]
async fn new_document(cxt: web::Data<SearchContext>, form: web::Json<Document>) -> HttpResponse {
    let elastic = cxt.get_cxt().read().await;
    let bucket_name = &form.bucket_uuid;
    let document_id = &form.document_md5_hash;
    let document_ref = &form.0;
    let to_value_result = serde_json::to_value(document_ref);
    if to_value_result.is_err() {
        let err = to_value_result.err().unwrap();
        let web_err = WebError::DocumentSerializing(err.to_string());
        return web_err.error_response();
    }

    let document_json = to_value_result.unwrap();
    let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);
    body.push(json!({"index": { "_id": document_id.as_str() }}).into());
    body.push(document_json.into());

    let response_result = elastic
        .bulk(BulkParts::Index(bucket_name.as_str()))
        .body(body)
        .send()
        .await;

    match response_result {
        Ok(_) => SuccessfulResponse::ok_response("Ok"),
        Err(err) => {
            let web_err = WebError::CreateDocument(err.to_string());
            web_err.error_response()
        }
    }
}

#[delete("/document/{bucket_name}/{document_id}")]
async fn delete_document(
    cxt: web::Data<SearchContext>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let elastic = cxt.get_cxt().read().await;
    let (bucket_name, document_id) = path.as_ref();
    let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
    let response_result = elastic
        .send(
            Method::Delete,
            s_path.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await;

    match response_result {
        Ok(_) => SuccessfulResponse::ok_response("Ok"),
        Err(err) => {
            let web_err = WebError::DeleteDocument(err.to_string());
            web_err.error_response()
        }
    }
}

#[get("/document/{bucket_name}/{document_id}")]
async fn get_document(
    cxt: web::Data<SearchContext>,
    path: web::Path<(String, String)>,
) -> WebResponse<web::Json<Document>> {
    let elastic = cxt.get_cxt().read().await;
    let (bucket_name, document_id) = path.as_ref();
    let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
    let response_result = elastic
        .send(
            Method::Get,
            s_path.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await;

    if response_result.is_err() {
        let err = response_result.err().unwrap();
        return Err(WebError::from(err));
    }

    let response = response_result.unwrap();
    let common_object = response.json::<Value>().await.unwrap();
    let document_json = &common_object[&"_source"];
    match Document::deserialize(document_json) {
        Ok(document) => Ok(web::Json(document)),
        Err(err) => Err(WebError::GetDocument(err.to_string())),
    }
}

fn deserialize_document(document_ref: &Document) -> Result<Value, WebError> {
    match serde_json::to_value(document_ref) {
        Ok(value) => Ok(value),
        Err(err) => Err(WebError::DocumentSerializing(err.to_string())),
    }
}

#[cfg(test)]
mod documents_endpoints {
    use crate::context::SearchContext;
    use crate::errors::{ErrorResponse, SuccessfulResponse};
    use crate::es_client::{build_elastic, build_service, init_service_parameters};
    use crate::wrappers::Document;

    use actix_web::test::TestRequest;
    use actix_web::{test, web, App};
    use serde_json::json;

    #[test]
    async fn build_application() {
        let service_parameters = init_service_parameters().unwrap();
        let es_host = service_parameters.es_host();
        let es_user = service_parameters.es_user();
        let es_passwd = service_parameters.es_passwd();
        let service_port = service_parameters.service_port();
        let service_addr = service_parameters.service_address();

        let elastic = build_elastic(es_host, es_user, es_passwd).unwrap();
        let cxt = SearchContext::_new(elastic);
        let app = App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service());

        let test_app = test::init_service(app).await;
        let test_bucket_name = "test_bucket";
        let test_document_name = "test_document";
        let test_document_path = "/tmp/dir/";
        let test_document_id = "79054025255fb1a26e4bc422aef54eb4";

        // Create new document with name: "test_document"
        let create_document_resp = TestRequest::post()
            .uri("/searcher/document/new")
            .set_json(&json!({
                "bucket_uuid": test_bucket_name,
                "bucket_path": "/tmp/test_document",
                "document_name": test_document_name,
                "document_path": test_document_path,
                "document_size": 1024,
                "document_type": "document",
                "document_extension": ".docx",
                "document_permissions": 777,
                "document_created": "2023-09-15T00:00:00Z",
                "document_modified": "2023-09-15T00:00:00Z",
                "document_md5_hash": test_document_id,
                "document_ssdeep_hash": "3a:34gh5",
                "entity_data": "Using skip_serializing does not skip deserializing the field.",
                "entity_keywords": ["document", "report"]
            }))
            .send_request(&test_app)
            .await;

        let new_document: SuccessfulResponse = test::read_body_json(create_document_resp).await;
        assert_eq!(new_document.code, 200);

        // Get document request by document name
        let get_document_resp = TestRequest::get()
            .uri(&format!(
                "/searcher/document/{}/{}",
                test_bucket_name, test_document_id
            ))
            .send_request(&test_app)
            .await;

        let get_document: Document = test::read_body_json(get_document_resp).await;
        assert_eq!(get_document.document_md5_hash, test_document_id);

        // Get document request by document name after updating
        let update_document_resp = TestRequest::put()
            .uri("/searcher/document/update")
            .set_json(&json!({
                "bucket_uuid": test_bucket_name,
                "bucket_path": "/tmp/test_document",
                "document_name": test_document_name,
                "document_path": "./",
                "document_size": 1024,
                "document_type": "document",
                "document_extension": ".docx",
                "document_permissions": 777,
                "document_created": "2023-09-15T00:00:00Z",
                "document_modified": "2023-09-15T00:00:00Z",
                "document_md5_hash": test_document_id,
                "document_ssdeep_hash": "3a:34gh5",
                "entity_data": "Using skip_serializing does not skip deserializing the field.",
                "entity_keywords": ["document", "report"]
            }))
            .send_request(&test_app)
            .await;

        let update_document: SuccessfulResponse = test::read_body_json(update_document_resp).await;
        assert_eq!(update_document.code, 200);

        let get_updated_document_resp = TestRequest::get()
            .uri(&format!(
                "/searcher/document/{}/{}",
                test_bucket_name, test_document_id
            ))
            .send_request(&test_app)
            .await;

        let get_document: Document = test::read_body_json(get_updated_document_resp).await;
        assert_eq!(get_document.document_path, "./");

        // Delete document by index
        let delete_document_resp = TestRequest::delete()
            .uri(&format!(
                "/searcher/document/{}/{}",
                test_bucket_name, test_document_id
            ))
            .send_request(&test_app)
            .await;

        let delete_document: SuccessfulResponse = test::read_body_json(delete_document_resp).await;
        assert_eq!(delete_document.code, 200);

        // Get document by index -> get error message
        let get_document_err_resp = TestRequest::get()
            .uri(&format!(
                "/searcher/document/{}/{}",
                test_bucket_name, "lsdfnbsikdjfsidg"
            ))
            .send_request(&test_app)
            .await;

        let get_document_err: ErrorResponse = test::read_body_json(get_document_err_resp).await;
        assert_eq!(get_document_err.code, 400);
    }

    #[test]
    async fn test_some() {
        let service_parameters = init_service_parameters().unwrap();
        let es_host = service_parameters.es_host();
        let es_user = service_parameters.es_user();
        let es_passwd = service_parameters.es_passwd();
        let service_port = service_parameters.service_port();
        let service_addr = service_parameters.service_address();

        let elastic = build_elastic(es_host, es_user, es_passwd).unwrap();
        let cxt = SearchContext::_new(elastic);
        let app = App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service());

        let entity_data_vec = vec![
            "СОГЛАСИЕ НА ОБРАБОТКУ ПЕРСОНАЛЬНЫХ ДАННЫХ № ____



В соответствии с Федеральным законом от 27.07.2006 № 152-ФЗ «О персональных данных» (далее – Закон о персональных данных) предоставляю оператору ГКУ «Инфогород», адрес местонахождения: 123112, г. Москва, 1-й Красногвардейский проезд, д. 21, стр. 1,  ИНН 7701356086/ ОГРН 5147746224324 (далее – Оператор) согласие на обработку (включая сбор, запись, систематизацию, накопление; хранение; уточнение (обновление, изменение), извлечение, использование, передачу, блокирование, удаление, уничтожение персональных данных иных действий, предусмотренных законодательством, совершаемых как  с использованием средств автоматизации, так и без использования таковых, в том объеме, который необходим для достижения целей обработки, указанных в настоящем согласии) моих персональных данных:
фамилия, имя, отчество;
должность и место работы;
адрес электронной почты;
номер телефона;
СНИЛС,
и подтверждаю, что, давая такое согласие, я действую свободно, своей волей и в своем интересе.
Цель обработки персональных данных: выполнение обязательств по государственному контракту от «14» августа 2023 г. на оказание комплексных услуг по технической поддержке  и адаптационному сопровождению Комплексной информационной системы «Государственные услуги в сфере образования в электронном виде» (КИС ГУСОЭВ) в части Сервиса «Портфолио учителя» №405/08/23 заключенному с Государственным казенным учреждением города Москвы «Информационный город» (ГКУ «Инфогород», ОГРН 5147746224324, ИНН 7701356086, адрес: г. Москва, 1-й Красногвардейский проезд, д. 21, стр. 1) в целях осуществления доступа  к информационным системам города Москвы, в том числе обрабатывающим персональные данные, ведения архивного делопроизводства в соответствии с внутренними нормативными документами ГКУ «Инфогород», и оказания услуг по указанному государственному контракту.
Настоящее согласие вступает в силу с даты подписания и действует в течение 10 лет.

Настоящее согласие может быть отозвано путем направления письменного заявления, составленного в соответствии со статьей 14 Закона о персональных данных Оператору по адресу, указанному в настоящем согласии, либо в виде электронного документа, подписанного в соответствии с требованиями, установленными законодательством Российской Федерации в области электронной подписи, на адрес электронной почты ig@it.mos.ru. Согласно статье 9 части 2 Закона о персональных данных согласие

______________________________________________________________/
	подпись		(Ф.И.О. полностью)
«___» ____________ 2023 г

",
            "СОГЛАСИЕ НА ОБРАБОТКУ ПЕРСОНАЛЬНЫХ ДАННЫХ № ____



В соответствии с Федеральным законом от 27.07.2006 № 152-ФЗ «О персональных данных» (далее – Закон о персональных данных) предоставляю оператору ГКУ «Инфогород», адрес местонахождения: 123112, г. Москва, 1-й Красногвардейский проезд, д. 21, стр. 1,  ИНН 7701356086/ ОГРН 5147746224324 (далее – Оператор) согласие на обработку (включая сбор, запись, систематизацию, накопление; хранение; уточнение (обновление, изменение), извлечение, использование, передачу, блокирование, удаление, уничтожение персональных данных иных действий, предусмотренных законодательством, совершаемых как  с использованием средств автоматизации, так и без использования таковых, в том объеме, который необходим для достижения целей обработки, указанных в настоящем согласии) моих персональных данных:
фамилия, имя, отчество;
должность и место работы;
адрес электронной почты;
номер телефона;
СНИЛС,
и подтверждаю, что, давая такое согласие, я действую свободно, своей волей и в своем интересе.
Цель обработки персональных данных: выполнение обязательств по государственному контракту от «14» августа 2023 г. на оказание комплексных услуг по технической поддержке  и адаптационному сопровождению Комплексной информационной системы «Государственные услуги в сфере образования в электронном виде» (КИС ГУСОЭВ) в части Сервиса «Портфолио учителя» №405/08/23 заключенному с Государственным казенным учреждением города Москвы «Информационный город» (ГКУ «Инфогород», ОГРН 5147746224324, ИНН 7701356086, адрес: 123112, г. Москва, 1-й Красногвардейский проезд, д. 21, стр. 1) в целях осуществления доступа  к информационным системам города Москвы, в том числе обрабатывающим персональные данные, ведения архивного делопроизводства в соответствии с внутренними нормативными документами ГКУ «Инфогород», и оказания услуг по указанному государственному контракту.
Настоящее согласие вступает в силу с даты подписания и действует в течение 5 лет.

Настоящее согласие может быть отозвано путем направления письменного заявления, составленного в соответствии со статьей 14 Закона о персональных данных Оператору по адресу, указанному в настоящем согласии, либо в виде электронного документа, подписанного в соответствии с требованиями, установленными законодательством Российской Федерации в области электронной подписи, на адрес электронной почты ig@it.mos.ru. Согласно статье 9 части 2 Закона о персональных данных согласие

______________________________________________________________/
	подпись		(Ф.И.О. полностью)
«___» ____________ 2023 г

",
            "Ростов вместе со своим командиром Василием Денисовым приезжает домой. Семейство Ростовых с радостью встречает Николая. После периода замалчивания высшее общество вырабатывает отношение к недавнему поражению русской армии, сваливая всю вину на австрийцев. Старый граф Ростов организует в Английском клубе обед в честь князя Багратиона. Пьер Безухов присутствует на обеде, недавно он получил анонимную записку, где прямо указывается на связь его жены с Долоховым. Долохов издевается над Пьером, поднимает тост «за здоровье красивых женщин и их любовников» и выхватывает из рук Пьера кантату, которую ему вручили как почётному гостю.",
            "В голове Пьера моментально складывается целая картина, он осознаёт что его жена - глупая, развратная женщина. В гневе Пьер вызывает Долохова на дуэль. На следующий день дуэлянты встречаются в лесу. Пьер, впервые взявший пистолет в руки, скашивает выстрелом опытного бретёра Долохова, ответный выстрел тяжелораненого Долохова проходит мимо. По просьбе Долохова его секундант Ростов спешит к его матери и с удивлением узнаёт, что буян и бретёр Долохов «жил в Москве со старушкой матерью и горбатой сестрой и был самый нежный сын и брат». Элен устраивает сцену Пьеру, тот приходит в ярость и едва не убивает её. Он выдаёт жене довереность на управление большей половиной своего состояния и уезжает из Москвы.",
            "Старый князь в Лысых Горах получил известие о гибели князя Андрея в Аустерлицком сражении, но вслед за ним приходит письмо от Кутузова, где фельдмаршал высказывает сомнения в смерти своего адьютанта. У жены князя Андрея начинаются роды. вместе с доктором в имение приезжает князь Андрей. Лиза рожает сына но умирает при родах. На её мёртвом лице Андрей читает укоризненное выражение: «Что вы со мной сделали?», которое впоследствии весьма долго не оставляет его. Новорождённому сыну дают имя Николай.",
            "LLM модель типа LLaMA-2, вышедшая в июле 2023 года, умеет многое с помощью правильно составленного текстового запроса (промта) без дополнительного программирования. Одна из очень полезных возможностей это суммаризация текста, c помощью которой можно сделать краткую выдержку по большому тексту даже на русском языке.",
            "Снизить в два раза количество ошибок при диагностировании сердечно-сосудистых заболеваний позволит математическая модель ученых СГМУ. Результаты опубликованы в журнале The European Physical Journal Special Topics. Сердце здорового человека бьется нерегулярно, то есть время сердечных сокращений может быть разным, объяснили ученые. При этом у людей с заболеваниями сердца и сосудов, например гипертонией, сердце бьется гораздо ровнее, чем у здоровых. По словам специалистов, степень нерегулярности сердечного ритма можно использовать для медицинской диагностики. Чаще всего для этого применяют спектральный анализ электрической активности сердца, полученный методом электрокардиографии (ЭКГ). Однако построение точных спектров биологических систем – крайне сложная задача, поэтому все диагностические методы предполагают погрешность, конкретная величина которой известна только очень приблизительно, отметили ученые. Исследователи Саратовского государственного медицинского университета им. В.И. Разумовского (СГМУ) создали математическую модель для точного вычисления погрешности при оценке спектров биологического сигнала. По их словам, применение полученных данных позволит в два раза снизить количество неверных сердечно-сосудистых диагнозов. «\"Принятые методы диагностики, например, гипертонии, как оказалось, дают верный диагноз только в 70% случаев. Основная причина высокой погрешности в том, что и мировые, и российские стандарты рекомендуют для анализа короткие электрокардиограммы в 3–5 минут. Наши расчеты показали, что данные ЭКГ за 15–20 минут помогут поднять это значение до 85%\", – сообщил старший научный сотрудник НИИ кардиологии СГМУ Юрий Ишбулатов. Question: Краткое изложение на русском языке.",
            "Вероятно самое просто это разбить документ по предложениям в размере менее 4К токенов и скармливать в промт. Получив все суммаризации по блокам, можно запустить еще одно разбиение на блоки, если финал превышает размер контекста. А если не превышает, то можно запустить финальную суммаризацию. Но я так еще не пробовал ). Также не забываем, что если требуется максимальное качество, можно пробовать версии 70B, а также само собой ChatGPT 4, у которого есть 32K контекст, и Claude 2 у которого есть контекст длинной 100К.",
            "Тико — застенчивая японская школьница. В свой одиннадцатый день рождения она получает в подарок от отца книгу. Открыв её, Тико выпускает озорную фею Тикл, которая была запечатана внутри книги, чтобы подшучивать над людьми. Сначала Тико не верит, что Тикл является феей, и требует, чтобы та доказала обратное. Так Тикл создаёт шарф по просьбе Тико как подарок Мико. Когда Тико понимает, что фея говорит правду, она объявляет, что была бы счастлива дружить с Тикл.",
            "Тикл использует своё волшебство для решения повседневных задач и, конечно, чтоб продолжать разыгрывать людей, что особенно раздражает младшую сестру Тико — Хину. Как и другие девочки-волшебницы, Тикл имеет специальную фразу, используемую для зачаровывания: «Махару Тамара Франпа». Хотя серия состоит в основном из веселых и причудливых историй, есть некоторые серьёзные моменты, хотя и незначительные.",
            "Обычно триграф рассматривается как сочетание трёх отдельных букв, что отражается в алфавитном порядке статей в словарях и справочниках. Однако в некоторых языках диграфы и триграфы имеют собственное место в алфавите, например, венгерский триграф dzs, обозначающий звук.",
            "Королевские военно-воздушные силы Канады отвечают за все полёты летательных аппаратов Канадских вооружённых сил, за безопасность воздушного пространства Канады, предоставляют самолёты для поддержки Королевского канадского военно-морского флота и Армии Канады. Являются партнёром Военно-воздушных сил США в защите воздушного пространства континента Северная Америка с помощью системы Командование воздушно-космической обороны Северной Америки (НОРАД). Обеспечивают также основные воздушные ресурсы и отвечают за Национальную программу поисково-спасательной службы.",
            "В 1968 году были соединены с Королевским канадским военно-морским флотом и Армией Канады, став частью объединения Канадских вооружённых сил. Военно-воздушные силы были разделены между несколькими различными командованиями: Командованием ПВО (ADC; перехватчики), Командованием транспортной авиации (ATC; авиаперевозки, поисково-спасательные службы), Мобильным командованием (истребители, вертолёты), Морским командованием (морские противолодочные и патрульные самолёты), а также Обучающим командованием (TC).",
            "Канадские ВВС были созданы в 1920 году в качестве преемника недолго существовавших ВВС из двух эскадрилий, образовавшихся во время Первой мировой войны в Европе, названных Канадскими ВВС. Новые ВВС, управляемые Авиационным Советом, были в значительной степени ориентированы на гражданские операции, такие как лесоводство, геодезия и патрулирование для борьбы с контрабандой. В 1923 году Авиационный Совет вошёл в Министерство национальной обороны, и год спустя Канадские ВВС получили королевский титул, став Королевскими военно-воздушными силами Канады. ВВС пострадали от бюджетного сокращения в начале 1930-х, но стали модернизироваться в это десятилетие. Однако к концу 1930-х годов ВВС не рассматриваются в качестве основной военной силы. С реализацией Плана Британского Содружества по авиационной подготовке во время Второй мировой войны ВВС были значительно расширены и стали четвёртыми по величине среди ВВС союзников. Во время войны ВВС были вовлечены в операции в Великобритании, в Северо-Западной Европе, Северной Атлантике, Египте, Италии, на Сицилии, Мальте, Шри-Ланке, в Индии, Мьянме, и в обороне своей страны.",
        ];

        let vec_hashes = vec![
            "96:q5U0fqukulD9o0fJJAHonINdONXlJT3tC3H6:wUATkulXvINdAX/k3H6",
            "96:q5U0fqukulD9o0fJJAHonItdONXlJT9tC3H6:wUATkulXvItdAXVk3H6",
            "24:HnGpwdbtfiCoWFzLIv8rewUOJ12Usjw1hIU03je/l1gJGv:HGSd0CoqT3Czw1SUGje3IY",
            "24:YLnz+/RIB00vrze93IwOWew8eb3GqrgorhNQIhBqkng/nXd/2SUQ7E3LsrPMN:gn6/w9vHk/Oe8eLGqrH2DdT0szi",
            "24:4PWjNwmhmxBSZXJaXTOl+Zk/KDwLHaUsbT0yRHUb1LTC5mm:ZixkZkHZkyDEpiORLTCEm",
            "12:wQkz7fSLk8whL2dlsXsdFeas7nHTqiBU0o3A0PfqJ87sbKT6Gyw/Le7Fw9s/Aak2:whTSAL2/2sC3zlXXJ8wbKmGPeOsc2",
            "96:nsU2kIWUsIYghr9zbdncCJyRPoGS9+a+t:nUkBchZzbdQR1S9Gt",
            "12:tCWM1qCW18f02PPyh5aMdE8U44VFf1baraueIk0lflFMBlg+Rn214eZPGYbkCi:tVEquXyLjE8U44VmrWIkU2vg+deZxu",
            "24:3gH0+cujeQ7/9bkM1oryL5TxeaCtV9T7L1Dtm:l+cujHbzLpxAV9T7VE",
            "12:3Rmc1W1QF1LsI15fv1gaN3wx3WdWzTx+egcCdRZZcdmOwNXK3Ygx+2W1FSYt:3b1WyQI1tW+yWdWhXkVcdmOeXKe2W1lt",
            "12:XVd/t/GmhxgJttljQCzqVCuOj/Qfpb1kved/NWr:XhhmJt7jQSF5b2pKo/+",
            "24:AzYhapdiyhaUl0dVO4NFRRRIGjCsJhFTZHLh4GtV3o9vZxn3ss:iYhevMO4dNjZJhtRykCB3",
            "24:qOC01zAZaZrn+oUl0brJhpf+xYR5+ablnqTcSI1M+SQkV0VPP:xC01AZaZrn+mvJhIxYRN52JIK+SQE0VH",
            "48:kzTD6KJgCpfrVCQjpG4rfpaEphnQi6kpm+2eXFjQb:knDpJLtrVCvehaEv96koi6b",
        ];

        let test_app = test::init_service(app).await;
        let test_bucket_name = "docs";
        for document_index in 1..15 {
            let document_size = 1024 + document_index * 10;
            let test_document_name = &format!("test_document_{}", document_index);
            let create_document_resp = TestRequest::post()
                .uri("/searcher/document/new")
                .set_json(&json!({
                    "bucket_uuid": test_bucket_name,
                    "bucket_path": "/tmp/test_document",
                    "document_name": test_document_name,
                    "document_path": "/tmp/dir/",
                    "document_size": document_size,
                    "document_type": "document",
                    "document_extension": ".docx",
                    "document_permissions": 777,
                    "document_created": "2023-09-15T00:00:00Z",
                    "document_modified": "2023-09-15T00:00:00Z",
                    "document_md5_hash": test_document_name,
                    "document_ssdeep_hash": vec_hashes.get(document_index),
                    "entity_data": entity_data_vec.get(document_index),
                    "entity_keywords": ["document", "report"]
                }))
                .send_request(&test_app)
                .await;
        }
    }
}
