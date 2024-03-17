use crate::endpoints::{CacherData, SearcherData};
use crate::errors::JsonResponse;
use crate::services::cacher::values::{CacherDocuments, CacherSearchParams};
use crate::services::CacherService;

#[cfg(feature = "enable-chunked")]
use crate::services::GroupedDocs;

use wrappers::document::Document;
use wrappers::search_params::SearchParams;

use actix_web::{post, web};

#[utoipa::path(
    post,
    path = "/similar/",
    tag = "Search similar documents by passed SearchParams",
    request_body = SearchParams,
    responses(
        (status = 200, description = "Successful", body = [Document]),
        (status = 401, description = "Failed while searching documents", body = ErrorResponse),
    )
)]
#[post("/")]
async fn search_similar_docs(
    cxt: SearcherData,
    cacher: CacherData,
    form: web::Json<SearchParams>,
) -> JsonResponse<Vec<Document>> {
    let client = cxt.get_ref();
    let search_form = form.0;

    #[cfg(feature = "disable-caching")]
    if cfg!(feature = "disable-caching") {
        return client.similarity(&search_form).await;
    }

    let cacher_params = CacherSearchParams::from(&search_form);
    match cacher
        .service
        .load::<CacherSearchParams, CacherDocuments>(cacher_params)
        .await
    {
        None => client.search(&search_form).await,
        Some(documents) => Ok(web::Json(documents.into())),
    }
}

#[cfg(feature = "enable-chunked")]
#[post("/")]
async fn search_similar_chunkec_docs(
    cxt: SearcherData,
    cacher: CacherData,
    form: web::Json<SearchParams>,
) -> JsonResponse<GroupedDocs> {
    let client = cxt.get_ref();
    let search_form = form.0;

    #[cfg(feature = "disable-caching")]
    if cfg!(feature = "disable-caching") {
        return client.similarity_chunked(&search_form).await;
    }

    let cacher_params = CacherSearchParams::from(&search_form);
    match cacher
        .service
        .load::<CacherSearchParams, CacherDocuments>(cacher_params)
        .await
    {
        None => client.similarity_chunked(&search_form).await,
        Some(documents) => {
            let grouped = client.group_document_chunks(documents.into());
            Ok(web::Json(grouped))
        }
    }
}

#[cfg(test)]
mod similar_endpoints {
    use crate::services::own_engine::context::OtherContext;
    use crate::services::SearcherService;

    use wrappers::document::{Document, DocumentBuilder};
    use wrappers::search_params::SearchParamsBuilder;

    use actix_web::test;

    const SSDEEP_HASH_CMP: &str =
        "12:JOGngjFtLax3bQrZvuwRZVZXwUSpUmHWAURnwP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y";

    #[test]
    async fn test_search_similar_docs() {
        let other_context = OtherContext::new("test".to_string());
        let mut search_params = SearchParamsBuilder::default()
            .query("unknown".to_string())
            .buckets(Some("test_bucket".to_string()))
            .document_type(String::default())
            .document_extension(String::default())
            .created_date_to(String::default())
            .created_date_from(String::default())
            .document_size_to(0)
            .document_size_from(0)
            .result_size(25)
            .result_offset(0)
            .build()
            .unwrap();

        let founded = other_context.similarity(&search_params).await;
        assert_eq!(founded.unwrap().len(), 0);

        let build_documents = create_documents_integration_test();
        for doc in build_documents.iter() {
            let _ = other_context.create_document(doc).await;
        }

        search_params.query = "unknown".to_string();
        let founded = other_context.similarity(&search_params).await;
        assert_eq!(founded.unwrap().len(), 0);

        search_params.query = SSDEEP_HASH_CMP.to_string();
        let founded = other_context.similarity(&search_params).await;
        assert_eq!(founded.unwrap().len(), 2);
    }

    #[test]
    async fn test_search_similar_docs_target() {
        let other_context = OtherContext::new("test".to_string());
        let mut search_params = SearchParamsBuilder::default()
            .query("unknown".to_string())
            .buckets(Some("test_bucket".to_string()))
            .document_type(String::default())
            .document_extension(String::default())
            .created_date_to(String::default())
            .created_date_from(String::default())
            .document_size_to(0)
            .document_size_from(0)
            .result_size(25)
            .result_offset(0)
            .build()
            .unwrap();

        let founded = other_context.similarity(&search_params).await;
        assert_eq!(founded.unwrap().len(), 0);

        let build_documents = create_documents_integration_test();
        for doc in build_documents.iter() {
            let _ = other_context.create_document(doc).await;
        }

        search_params.query = "unknown".to_string();
        let founded = other_context.similarity(&search_params).await;
        assert_eq!(founded.unwrap().len(), 0);

        search_params.query = SSDEEP_HASH_CMP.to_string();
        let founded = other_context.similarity(&search_params).await;
        assert_eq!(founded.unwrap().len(), 2);
    }

    fn create_documents_integration_test() -> Vec<Document> {
        let entity_data_vec = vec![
            "Ростов вместе со своим командиром Василием Денисовым приезжает домой. Семейство Ростовых с удовольствием встречает Николая. После периода замалчивания высшее общество проявляет отношение к недавнему поражению русской армии, сваливая всю вину на австрийцев. Старый граф Ростов ведет обед в Английском клубе в честь князя Багратиона. Пьер Безухов сидит на обеде, недавно он получил анонимную записку, где поворачивает прямо на связь его жены с Долоховым. Долохов издевается над Пьером, поднимает тост «за здоровье красивых женщин и их любовников» и выхватывает из рук Пьера кантату, который ему вручили как почётному гостю.",
            "Rostov, together with his commander Vasily Denisov, comes home. The Rostov family happily welcomes Nikolai. After a period of silence, high society develops an attitude towards the recent defeat of the Russian army, placing all the blame on the Austrians. The old Count of Rostov organizes a dinner in honor of Prince Bagration at the English Club. Pierre Bezukhov is present at the dinner; he recently received an anonymous note, which directly indicates his wife’s connection with Dolokhov. Dolokhov mocks Pierre, raises a toast “to the health of beautiful women and their lovers” and snatches the cantata from Pierre’s hands, which he was given as an honored guest.",
            "皮埃尔的脑海中立刻浮现出一幅完整的画面；他意识到他的妻子是一个愚蠢、堕落的女人。 皮埃尔愤怒地向多洛霍夫发起决斗挑战。 第二天，决斗者在森林里相遇。 第一次拿起手枪的皮埃尔，一枪将经验丰富的战士多洛霍夫砍倒，重伤的多洛霍夫的回击擦肩而过。 在多洛霍夫的要求下，他的第二个罗斯托夫急忙去找他的母亲，惊讶地发现，爱打闹、粗暴的多洛霍夫“和一位年老的母亲和一个驼背的妹妹住在莫斯科，是最温柔的儿子和兄弟”。 海伦娜对皮埃尔大吵大闹，皮埃尔勃然大怒，差点杀了她。 他授权妻子管理他一半以上的财产，然后离开莫斯科。",
            "Старый князь в Лысых Горах получил известие о гибели князя Андрея в Аустерлицком сражении, но вслед за ним приходит письмо от Кутузова, где фельдмаршал высказывает сомнения в смерти своего адьютанта. У жены князя Андрея начинаются роды. вместе с доктором в имение приезжает князь Андрей. Лиза рожает сына но умирает при родах. На её мёртвом лице Андрей читает укоризненное выражение: «Что вы со мной сделали?», которое впоследствии весьма долго не оставляет его. Новорождённому сыну дают имя Николай.",
            "An LLM model like LLaMA-2, released in July 2023, can do a lot using a correctly composed text query (prompt) without additional programming. One of the very useful crates is text summarization, with which you can make a short summary of a large text, even in Russian.",
            "The mathematical model of SSMU scientists allows us to halve the number of errors in diagnosing cardiovascular diseases. The results were published in The European Physical Journal Special Topics. The heart of a healthy person beats irregularly, that is, the timing of heart contractions can be different, scientists explained. According to experts, the degree of irregularity of heart rhythm can be used for medical purposes. ). accurate calculation of errors when measuring the spectra of a biological signal. According to them, the application of the obtained data will halve the number of incorrect cardiovascular diseases. “Accepted diagnostic methods, for example, of hypertension, as it turned out, give the correct diagnosis only in 70% of cases. The main reason for the high error is that both world and Russian standards use 3-5 minutes to analyze an electrocardiogram. Our calculations indicate that ECG data within 15–20 minutes can increase this value to 85%,” said Yuri Ishbulatov, senior researcher at the Research Institute of Cardiology of SSMU. Question: Brief summary in English.",
            "Probably the simplest thing is to split the document into proposals of less than 4K tokens and feed it into the industrial sector. Once you have all the block sums, you can run another block split if the final is larger than the context size. And if it does not exceed, then you can run the final summation. But I haven't tried that yet). Also, do not forget that if maximum quality is required, you can try versions 70B, as well as ChatGPT 4, which has a 32K context, and Claude 2, which has a 100K long context.",
            "Вероятно, самое простое — разделить документ на предложения размером менее 4 тыс. токенов и передать его промышленному сектору. Когда у вас есть все суммы блоков, вы можете запустить еще одно разделение блоков, если итоговый результат превышает размер контекста. А если не превышает, то можно провести итоговое суммирование. Но я еще не пробовал). Также не забывайте, что если требуется максимальное качество, можно попробовать версии 70B, а также ChatGPT 4, имеющий контекст длиной 32К, и Claude 2, имеющий контекст длиной 100К.",
            "Тіко - сором'язлива японська школярка. У свій одинадцятий день народження вона одержує у подарунок від батька книгу. Відкривши її, Тіко випускає бешкетну фею Тікл, яка була запечатана всередині книги, щоб жартувати над людьми. Спочатку Тіко не вірить, що Тикл є феєю, і вимагає, щоб та довела протилежне. Так Тикл створює шарф на прохання Тіко як подарунок Міко. Коли Тіко розуміє, що фея говорить правду, вона оголошує, що була б щаслива дружити з Тіклом.",
            "Тикл использует своё волшебство для решения повседневных задач и, конечно, чтоб продолжать разыгрывать людей, что особенно раздражает младшую сестру Тико — Хину. Как и другие девочки-волшебницы, Тикл имеет специальную фразу, используемую для зачаровывания: «Махару Тамара Франпа». Хотя серия состоит в основном из веселых и причудливых историй, есть некоторые серьёзные моменты, хотя и незначительные.",
            "Typically, a trigraph is viewed as a combination of three individual letters, which is reflected in the alphabetical order of entries in dictionaries and reference books. However, in some languages, digraphs and trigraphs have their own place in the alphabet, for example, the Hungarian trigraph dzs, which denotes sound.",
            "Королевские военно-воздушные силы Канады отвечают за все полёты летательных аппаратов Канадских вооружённых сил, за безопасность воздушного пространства Канады, предоставляют самолёты для поддержки Королевского канадского военно-морского флота и Армии Канады. Являются партнёром Военно-воздушных сил США в защите воздушного пространства континента Северная Америка с помощью системы Командование воздушно-космической обороны Северной Америки (НОРАД). Обеспечивают также основные воздушные ресурсы и отвечают за Национальную программу поисково-спасательной службы.",
            "В 1968 були пов'язані з Королівським канадським військово-морським флотом і Армією Канади, ставши частиною об'єднання Канадських збройних сил. Військово-повітряні сили були розділені між кількома різними командуваннями: Командуванням ППО (ADC; перехоплювачі), Командуванням транспортної авіації (ATC; авіаперевезення, пошуково-рятувальні служби), Мобільним командуванням (винищувачі, вертольоти), Морським командуванням (морські протичовнові та патрульні літаки) , а також Навчальним командуванням (TC).",
            "加拿大空军成立于 1920 年，是欧洲第一次世界大战期间成立的短命的两中队空军（称为加拿大空军）的继承者。 由空军委员会控制的新空军主要专注于林业、测量和反走私巡逻等民事行动。 1923年，航空委员会并入国防部，一年后加拿大空军获得皇家称号，成为加拿大皇家空军。 空军在 20 世纪 30 年代初遭受预算削减，但在这十年中开始现代化。 然而，到 20 世纪 30 年代末，空军已不再被视为一支主要军事力量。 第二次世界大战期间，随着英联邦空中训练计划的实施，空军大大扩充，成为盟军第四大空军。 战争期间，英国皇家空军参与了英国、西北欧、北大西洋、埃及、意大利、西西里岛、马耳他、斯里兰卡、印度、缅甸和本土防御的行动。",
            "Канадські ВПС були створені в 1920 році як наступник ВВС, що недовго існували, з двох ескадрилій, що утворилися під час Першої світової війни в Європі, названих Канадськими ВПС. Нові ВПС, керовані Авіаційною Радою, були значною мірою орієнтовані на цивільні операції, такі як лісівництво, геодезія та патрулювання для боротьби з контрабандою. У 1923 році Авіаційна Рада увійшла до Міністерства національної оборони, і через рік Канадські ВПС отримали королівський титул, ставши Королівськими військово-повітряними силами Канади. ВПС постраждали від бюджетного скорочення на початку 1930-х, але стали модернізуватися цього десятиліття. Однак до кінця 1930-х років ВПС не розглядаються як основна військова сила. З реалізацією Плану Британської Співдружності з авіаційної підготовки під час Другої світової війни ВПС були значно розширені та стали четвертими за величиною серед ВПС союзників. Під час війни ВПС були залучені в операції у Великій Британії, у Північно-Західній Європі, Північній Атлантиці, Єгипті, Італії, на Сицилії, Мальті, Шрі-Ланці, в Індії, М'янмі та в обороні своєї країни.",
            "Payment. The lease fee shall be paid by the Tenant in a lump sum for the whole term of Lease Agreement, upon signing the Agreement in the amount of 100 per each day of lease. The total amount of lease fee hereunder shall constitute 30. Security deposit. The Tenant shall make to the Landlord, whereas the Landlord shall accept from the Tenant the Security Deposit in the amount of 200 dollars intended to cover any damages as a result of direct action, omissions or negligence of the Tenant. Upon termination of the Lease Agreement, the Security Deposit shall be refunded to the Tenant, less any expenses to cover the damages inflicted by the Tenant. If the Tenant loses any keys from the leased property, the Tenant shall pay to the Landlord the amount of 3000 (three thousand) dollars.",
            "Payment. The lease fee shall be paid by the Tenant in a lump sum for the whole term of Lease Agreement, upon signing the Agreement in the amount of 50 per each day of lease. The total amount of lease fee hereunder shall constitute 10. Security deposit. The Tenant shall make to the Landlord, whereas the Landlord shall accept from the Tenant the Security Deposit in the amount of 100 dollars intended to cover any damages as a result of direct action, omissions or negligence of the Tenant. Upon termination of the Lease Agreement, the Security Deposit shall be refunded to the Tenant, less any expenses to cover the damages inflicted by the Tenant. If the Tenant loses any keys from the leased property, the Tenant shall pay to the Landlord the amount of 1500 (on thousand and five hundred) dollars.",
            "Оплата. Арендная плата вносится Арендатором единовременно за весь срок Договора аренды, при подписании Договора, в размере 500 за каждый день аренды. Общая сумма арендной платы по настоящему договору составляет 30. Гарантийный депозит. Арендатор обязан внести Арендодателю, а Арендодатель обязан принять от Арендатора Гарантийный депозит в размере 15000 рублей, предназначенный для покрытия любого ущерба, возникшего в результате прямых действий, бездействия или халатности Арендатора. При расторжении Договора аренды Гарантийный депозит возвращается Арендатору за вычетом расходов на покрытие причиненного Арендатором ущерба. В случае утраты Арендатором ключей от арендуемого имущества, Арендатор обязан выплатить Арендодателю сумму в размере 15000 (пятнадцать тысячь) рублей.",
            "Оплата. Арендная плата вносится Арендатором единовременно за весь срок Договора аренды, при подписании Договора, в размере 200 за каждый день аренды. Общая сумма арендной платы по настоящему договору составляет 30. Гарантийный депозит. Арендатор обязан внести Арендодателю, а Арендодатель обязан принять от Арендатора Гарантийный депозит в размере 6000 рублей, предназначенный для покрытия любого ущерба, возникшего в результате прямых действий, бездействия или халатности Арендатора. При расторжении Договора аренды Гарантийный депозит возвращается Арендатору за вычетом расходов на покрытие причиненного Арендатором ущерба. В случае утраты Арендатором ключей от арендуемого имущества, Арендатор обязан выплатить Арендодателю сумму в размере 10000 (десять тысячь) рублей.",
        ];

        let vec_hashes = vec![
            "24:HnGpwdFdiCoWarOIv8rewUClgJPYsji11QhIU03je/l1gJuv:HGSdGCo7UxSJiYSUGje3Iw",
            "12:OdI5Kuq7IeEp/hAQ3XkMfQqhz2eWBhF5Ywjh90FKnHgxn5RQM4feDP:kI56tnkXkbqqPF5GDUfeb",
            "12:8iztGNnUwXxGA654YuA2r2Uwg/V0JmkMISvpI3bEFUpzZQIuLfC9iP2J:JBsnUwha5XUL/VXkrSvpUbEFWzCdDm4a",
            "24:4PWjNwmhmxBSZXJaXTOl+Zk/KDwLHaUsbT0yRHUb1LTC5mm:ZixkZkHZkyDEpiORLTCEm",
            "6:Co7EdVG61RspPLELJIPyR0c8B4G0ZNZ8A6u6mq7KLVLiL:TaVLqcgo0c8/mqE6",
            "24:4S9Ad2fRAwZP1n6K7li+LoFxvw+IVUfIp9Fi0wL:2uRAwZP16Ks/4BQIpaL",
            "12:K9y92A35iKP2qQkgNsAZANuGAVGperdQh:Kc92AnOqZgNsASwAeM",
            "12:fV0guz18gCOChnay1dcW15ZmMdE8/LFf1baIrAtLfl+l2f+RntbNb2fbkSviM2ft:f3u+wCJamdcy5FE8Pm7/WM+T5CTQTJ",
            "24:mtHv5OWcgwebYMJEt2AfapeLFJ3vVebYlcVJ9quxZ:ouaYMJUBJ9aYlcVfq+Z",
            "12:3Rmc1W1QF1LsI15fv1gaN3wx3WdWzTx+egcCdRZZcdmOwNXK3Ygx+2W1FSYt:3b1WyQI1tW+yWdWhXkVcdmOeXKe2W1lt",
            "6:eMRLcPmW9wRFr1sPNHtKSgi6Y/rHGgXJ3jgICN9/sgno3N4bWJQHRUgeBL:eMiOTRFWR0Sgi3tUsgXy6HK",
            "24:AzYhapdiyhaUl0dVO4NFRRRIGjCsJhFTZHLh4GtV3o9vZxn3ss:iYhevMO4dNjZJhtRykCB3",
            "24:T4cg2fnJ9461O4eM6j+EaZMGVysGnvA3ME1GsZP:TvlffQ4z6j+EaZMSysyAGsd",
            "12:c+RLKK+HdfKz6/mh2GKcROC12UZlu+V1u7E9lOT5OPtk1uPwdMasEo/3zU:c+RLKKxzkGDRV1VbXu7arPGuPwdQ/3A",
            "48:nLtYHHUSwgZMQBiiC9VuNRmdVaogI28NLKR:YUmMSihVW2YI28Nq",
            "12:JOGngjFt1Hax3bQbvuwRZVZXwSpUmHWAURnwP+EfzRR00C1U9qA:DIF/pukvZgRWWATP+Eo71vA",
            "12:JOGngjFtLax3bQrZvuwRZVZXwUSpUmHWAURnwP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y",
            "24:Sbo6SpDD+amr7xlm1ECVRiHN6irJi37RidE8NbRtOtLWAD0RidvOCrS1BrsipaEy:S7ufJsmWCVgHNDu7gdEEzAIgdmC2/rvy",
            "24:Sbo6SpDD+alr7xlm1ECVRiHN6irJi37Rid3NbRtOtLWAD0RidvOCrS1BrsipaEMb:S7ufJtmWCVgHNDu7gdlzAIgdmC2/rvaN",
        ];

        let mut build_documents = Vec::<Document>::new();
        let test_bucket_name = "test_bucket";
        for document_index in 1..17 {
            let document_size = 1024 + document_index * 10;
            let test_document_name = &format!("test_document_{}", document_index);
            let ssdeep_hash = *vec_hashes.get(document_index as usize).unwrap();
            let entity_data = *entity_data_vec.get(document_index as usize).unwrap();
            let document = DocumentBuilder::default()
                .bucket_uuid(test_bucket_name.to_string())
                .bucket_path("/".to_string())
                .document_name(test_document_name.clone())
                .document_path("/".to_string())
                .document_size(document_size)
                .document_type("document".to_string())
                .document_extension(".txt".to_string())
                .document_permissions(document_size)
                .document_md5(test_document_name.clone())
                .document_ssdeep(ssdeep_hash.to_string())
                .content_md5(test_document_name.clone())
                .content_uuid(hasher::gen_uuid())
                .content(entity_data.to_string())
                .content_vector(Vec::default())
                .highlight(None)
                .document_created(None)
                .document_modified(None)
                .build()
                .unwrap();

            build_documents.push(document);
        }

        build_documents
    }
}
