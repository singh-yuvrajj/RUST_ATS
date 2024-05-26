// #[allow(unused_imports)]

// use langchain_rust::{language_models::llm::LLM, llm::openai::OpenAI};

// use dotenv::dotenv;


// use langchain_rust::embedding::{Embedder, EmbeddingModel, FastEmbed, openai::OpenAiEmbedder,InitOptions, TextEmbedding};
// use langchain_rust::document_loaders::lo_loader::LoPdfLoader;
// use langchain_rust::document_loaders::Loader;
// use futures_util::StreamExt;
// use langchain_rust::schemas::Document;
// use langchain_rust::semantic_router::utils::{ cosine_similarity , combine_embeddings};

// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let path = "./data/Resume.pdf";

//     let loader = LoPdfLoader::from_path(path).expect("Failed to create PdfLoader");

//     let resume = loader
//         .load()
//         .await
//         .unwrap()
//         .map(|d| d.unwrap())
//         .collect::<Vec<Document>>()
//         .await;

//     let job_description = format!(
//         "role: {role}\n\
//         description: {description}\n\
//         location: {location}\n\
//         qualification: {qualification}\n\
//         company_id: {company_id}\n\
//         issued_on: {issued_on}\n\
//         validity: {validity}",
//         role = "Software Development Engineer - Dev Platform",
//         description = "At Workday, it all began with a conversation over breakfast. When our founders met at a sunny California diner, they came up with an idea to revolutionize the enterprise software market. And when we began to rise, one thing that really set us apart was our culture. A culture which was driven by our value of putting our people first. And ever since, the happiness, development, and contribution of every Workmate is central to who we are. Our Workmates believe a healthy employee-centric, collaborative culture is the essential mix of ingredients for success in business. That’s why we look after our people, communities and the planet while still being profitable. Feel encouraged to shine, however that manifests: you don’t need to hide who you are. You can feel the energy and the passion, it's what makes us unique. Inspired to make a brighter work day for all and transform with us to the next stage of our growth journey? Bring your brightest version of you and have a brighter work day here.\n\
//         About the Team\n\
//         Workday is leading Enterprise Cloud Applications provider developing and deploying a wide range of business applications. OMS team provides Developer Platform for Product teams to Design & Develop outstanding products. We provide development languages and frameworks, as well handles life cycle of generated Metadata. If you appreciate the complexity of a highly available, scalable distributed platform which involves storing & retrieving sophisticated data and wonder about all the things needed to support growing number of enterprise customers, give us a shout!\n\
//         About the Role\n\
//         As a member of this team, you’ll work with outstandingly innovative and experienced engineers to address growing scale & performance needs of OMS Dev Platform, while improving its capabilities to better support Product pillars! You will address outstanding challenges as Workday continues to onboard larger customers, supports more customizations and partners with new product acquisitions.\n\
//         Collaborate with multi-functional teams to add new feature and functionalities to the platform\n\
//         Craft and develop software systems with low latency and memory footprint that support high transactional volume\n\
//         Produce documentation/usage guide with reference implementation and use cases\n\
//         Perform peer code reviews as part of everyday workflow\n\
//         Maintain a highly stable framework by prioritizing & fixing Production and Pipeline issues\n\
//         Contribute ideas for continually improving the team's efficiency, job enjoyment, and code quality\n\
//         About You\n\
//         You are a self-motivated & organized individual with keen curiosity about learning & growth. You have experience with verifying distributed systems, debugging pipeline & production issues.\n\
//         Basic Qualifications- Software Development Engineer\n\
//         4+ years of software engineering experience\n\
//         4+ years of experience working with Java and related technologies\n\
//         Basic Qualifications- Sr. Associate Software Development Engineer\n\
//         3+ years of software engineering experience\n\
//         3+ years of experience working with Java and related technologies\n\
//         Other Qualifications\n\
//         BS or MS degree in Computer Science, Math, or related field\n\
//         Knowledge of common design patterns, concurrency, multithreading, server architectures, and distributed systems\n\
//         Proven track record of working in multi-functional teams and agile delivery\n\
//         Excellent verbal and written communication skills\n\
//         Experience in a fast-paced work environment\n\
//         Strong problem solving and critical thinking skills\n\
//         Internal candidates: Demonstrated understanding of XO, YP, WATS, WARP, SkyLab\n\
//         Workday Pay Transparency Statement\n\
//         The annualized base salary ranges for the primary location and any additional locations are listed below.  Workday pay ranges vary based on work location. As a part of the total compensation package, this role may be eligible for the Workday Bonus Plan or a role-specific commission/bonus, as well as annual refresh stock grants. Recruiters can share more detail during the hiring process. Each candidate’s compensation offer will be based on multiple factors including, but not limited to, geography, experience, skills, job duties, and business need, among other things. For more information regarding Workday’s comprehensive benefits, please click here.\n\
//         Primary Location: USA.CO.Boulder\n\
//         Primary Location Base Pay Range:  $124,000 USD - $186,000 USD\n\
//         Additional US Location(s) Base Pay Range: $117,800 USD - $210,000 USD\n\
//         Our Approach to Flexible Work\n\
//         With Flex Work, we’re combining the best of both worlds: in-person time and remote. Our approach enables our teams to deepen connections, maintain a strong community, and do their best work. We know that flexibility can take shape in many ways, so rather than a number of required days in-office each week, we simply spend at least half (50%) of our time each quarter in the office or in the field with our customers, prospects, and partners (depending on role). This means you'll have the freedom to create a flexible schedule that caters to your business, team, and personal needs, while being intentional to make the most of time spent together. Those in our remote 'home office' roles also have the opportunity to come together in our offices for important moments that matter.",
//         location = "USA, CO, Boulder",
//         qualification = "BS or MS degree in Computer Science, Math, or related field",
//         company_id = 101,
//         issued_on = "2023-01-15",
//         validity = 365
//     );
    

//     // let resume_strings: Vec<String> = resume.iter().map(|doc| doc.page_content.clone()).collect();

//     let resume_strings = resume.iter()
//     .map(|doc| doc.page_content.clone())
//     .collect::<Vec<_>>()
//     .join(" ");

//     // let fastembed = FastEmbed::try_new().unwrap();

//     // let model = TextEmbedding::try_new(InitOptions {
//     //     model_name: EmbeddingModel::AllMiniLML6V2,
//     //     show_download_progress: true,
//     //     ..Default::default()
//     // })
//     // .unwrap();

//     // let fastembed = FastEmbed::from(model);


//     let chunk_size = 7000; // Adjust based on your token limit
//     let chunks = chunk_string(&resume_strings, chunk_size);

//     dotenv().ok();
//     let openai = OpenAiEmbedder::default();

//     let temp_resume_embeddings = embed_chunks(chunks).await?;

//     let resume_embeddings = combine_embeddings(&temp_resume_embeddings);

//     let jd_embeddings = openai.embed_query(&job_description).await?;

//     let similarity = cosine_similarity(&jd_embeddings, &resume_embeddings) * 100.0;
//     println!("Cosine similarity with resume: {} %",similarity);

//     Ok(())
// }


// fn chunk_string(s: &str, chunk_size: usize) -> Vec<String> {
//     s.chars()
//         .collect::<Vec<_>>()
//         .chunks(chunk_size)
//         .map(|chunk| chunk.iter().collect())
//         .collect()
// }

// async fn embed_chunks(chunks: Vec<String>) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
//     dotenv().ok();
//     let openai = OpenAiEmbedder::default();
//     let mut embeddings = Vec::new();
    
//     for chunk in chunks {
//         let embedding = openai.embed_query(&chunk).await?;
//         embeddings.push(embedding);
//     }
    
//     Ok(embeddings)
// }





#[allow(unused_imports)]

use dotenv::dotenv;
use std::error::Error;
use tokio::main;

mod loader;
mod embedder;

use loader::load_resume;
use embedder::{chunk_string, embed_chunks };
use langchain_rust::semantic_router::utils::{ cosine_similarity , combine_embeddings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let path = "./data/Resume2.pdf";
    let resume = load_resume(path).await?;

    let job_description = format!(
        "role: {role}\n\
        description: {description}\n\
        location: {location}\n\
        qualification: {qualification}\n\
        company_id: {company_id}\n\
        issued_on: {issued_on}\n\
        validity: {validity}",
        role = "Software Development Engineer - Dev Platform",
        description = "At Workday, it all began with a conversation over breakfast. When our founders met at a sunny California diner, they came up with an idea to revolutionize the enterprise software market. And when we began to rise, one thing that really set us apart was our culture. A culture which was driven by our value of putting our people first. And ever since, the happiness, development, and contribution of every Workmate is central to who we are. Our Workmates believe a healthy employee-centric, collaborative culture is the essential mix of ingredients for success in business. That’s why we look after our people, communities and the planet while still being profitable. Feel encouraged to shine, however that manifests: you don’t need to hide who you are. You can feel the energy and the passion, it's what makes us unique. Inspired to make a brighter work day for all and transform with us to the next stage of our growth journey? Bring your brightest version of you and have a brighter work day here.\n\
        About the Team\n\
        Workday is leading Enterprise Cloud Applications provider developing and deploying a wide range of business applications. OMS team provides Developer Platform for Product teams to Design & Develop outstanding products. We provide development languages and frameworks, as well handles life cycle of generated Metadata. If you appreciate the complexity of a highly available, scalable distributed platform which involves storing & retrieving sophisticated data and wonder about all the things needed to support growing number of enterprise customers, give us a shout!\n\
        About the Role\n\
        As a member of this team, you’ll work with outstandingly innovative and experienced engineers to address growing scale & performance needs of OMS Dev Platform, while improving its capabilities to better support Product pillars! You will address outstanding challenges as Workday continues to onboard larger customers, supports more customizations and partners with new product acquisitions.\n\
        Collaborate with multi-functional teams to add new feature and functionalities to the platform\n\
        Craft and develop software systems with low latency and memory footprint that support high transactional volume\n\
        Produce documentation/usage guide with reference implementation and use cases\n\
        Perform peer code reviews as part of everyday workflow\n\
        Maintain a highly stable framework by prioritizing & fixing Production and Pipeline issues\n\
        Contribute ideas for continually improving the team's efficiency, job enjoyment, and code quality\n\
        About You\n\
        You are a self-motivated & organized individual with keen curiosity about learning & growth. You have experience with verifying distributed systems, debugging pipeline & production issues.\n\
        Basic Qualifications- Software Development Engineer\n\
        4+ years of software engineering experience\n\
        4+ years of experience working with Java and related technologies\n\
        Basic Qualifications- Sr. Associate Software Development Engineer\n\
        3+ years of software engineering experience\n\
        3+ years of experience working with Java and related technologies\n\
        Other Qualifications\n\
        BS or MS degree in Computer Science, Math, or related field\n\
        Knowledge of common design patterns, concurrency, multithreading, server architectures, and distributed systems\n\
        Proven track record of working in multi-functional teams and agile delivery\n\
        Excellent verbal and written communication skills\n\
        Experience in a fast-paced work environment\n\
        Strong problem solving and critical thinking skills\n\
        Internal candidates: Demonstrated understanding of XO, YP, WATS, WARP, SkyLab\n\
        Workday Pay Transparency Statement\n\
        The annualized base salary ranges for the primary location and any additional locations are listed below.  Workday pay ranges vary based on work location. As a part of the total compensation package, this role may be eligible for the Workday Bonus Plan or a role-specific commission/bonus, as well as annual refresh stock grants. Recruiters can share more detail during the hiring process. Each candidate’s compensation offer will be based on multiple factors including, but not limited to, geography, experience, skills, job duties, and business need, among other things. For more information regarding Workday’s comprehensive benefits, please click here.\n\
        Primary Location: USA.CO.Boulder\n\
        Primary Location Base Pay Range:  $124,000 USD - $186,000 USD\n\
        Additional US Location(s) Base Pay Range: $117,800 USD - $210,000 USD\n\
        Our Approach to Flexible Work\n\
        With Flex Work, we’re combining the best of both worlds: in-person time and remote. Our approach enables our teams to deepen connections, maintain a strong community, and do their best work. We know that flexibility can take shape in many ways, so rather than a number of required days in-office each week, we simply spend at least half (50%) of our time each quarter in the office or in the field with our customers, prospects, and partners (depending on role). This means you'll have the freedom to create a flexible schedule that caters to your business, team, and personal needs, while being intentional to make the most of time spent together. Those in our remote 'home office' roles also have the opportunity to come together in our offices for important moments that matter.",
        location = "USA, CO, Boulder",
        qualification = "BS or MS degree in Computer Science, Math, or related field",
        company_id = 101,
        issued_on = "2023-01-15",
        validity = 365
    );

    let resume_strings = resume.iter()
        .map(|doc| doc.page_content.clone())
        .collect::<Vec<_>>()
        .join(" ");

    let chunk_size = 7000; // Adjust based on your token limit
    let resume_chunks = chunk_string(&resume_strings, chunk_size);
    let job_chunks = chunk_string(&job_description, chunk_size);

    let temp_resume_embeddings = embed_chunks(resume_chunks).await?;
    let resume_embeddings = combine_embeddings(&temp_resume_embeddings);

    let temp_jd_embeddings = embed_chunks(job_chunks).await?;
    let jd_embeddings = combine_embeddings(&temp_jd_embeddings);


    let similarity = cosine_similarity(&jd_embeddings, &resume_embeddings) * 100.0;
    println!("Cosine similarity with resume: {} %", similarity);

    Ok(())
}
