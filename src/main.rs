use gemini_ai::{
    cloud::cloud::create_cloud_gemini_and_get_upload_uri,
    format::{key, nested, Function, Parameters, PropertiesParameter},
    pulse::{
        format::{feature, Properties},
        pulse::{GeminiPulse, GeminiPulseBuilder},
    },
    schema::schema_store,
    GeminiContentGenBuilder, Pair,
};

fn main() {
    let property = nested(
        "achivements",
        &[
            Pair {
                key: "year",
                r#type: "STRING",
            },
            Pair {
                key: "achivement",
                r#type: "STRING",
            },
        ],
    );

    let schema = schema_store(&[property]);

    let builder = GeminiContentGenBuilder::new()
        .env("GEMINI_API_KEY")
        .model(gemini_ai::Models::GEMINI_1_5_PRO_002)
        .kind(gemini_ai::Kind::Text)
        .instruction("response the current day")
        .text("current temperature in poindicherry")
        .max_token(gemini_ai::TokenLen::Default)
        .build()
        .output();

    println!("{}", builder);
    let builder = GeminiContentGenBuilder::new()
        .env("GEMINI_API_KEY")
        .model(gemini_ai::Models::GEMINI_1_5_PRO_002)
        .kind(gemini_ai::Kind::Text)
        .instruction("response the current day")
        .text("current temperature in poindicherry")
        .max_token(gemini_ai::TokenLen::Default)
        .build()
        .output();
    println!("{}", builder);

    // create_cloud_gemini_and_get_upload_uri("statics/video.mp4");
    // // let string = decode_gemini(&builder);
    // // println!("{:?}", string);
    // check_state();
    // let paramter = PropertiesParameter::new("trun_on_light", "string", "turn_on_the_led");
    // let paramterss = Properties::parameter("string", &paramter, Some(&["turn_on_the_led"]));
    // let parameter1 = Parameters::parameter("turn_on_the_led", "billionaire", &paramterss);

    // let paramter = PropertiesParameter::new("trun_on_light", "string", "trun_on_light");
    // let paramterss = Properties::parameter("string", &paramter, Some(&["turn_on_light"]));
    // let parameter2 = Parameters::parameter("turn_on_light", "billionaire", &paramterss);

    // let paramter = PropertiesParameter::new("trun_on_light", "string", "turn_on_the_led");
    // let paramterss = Properties::parameter("string", &paramter, Some(&["turn_on_light"]));
    // let parameter3 = Parameters::parameter("turn_on_light", "billionaire", &paramterss);

    // let response = Function::new(&[parameter1, parameter2, parameter3]);
    // println!("{}", response);

    // let properties = Properties{
    //     property:""
    // }
    // let paramter = PropertiesParameter {
    //     description: Some(""),
    //     r#type: "",
    // };
    // let train = Function {
    //     name: "automation",
    //     desciption: "used to automate the process",
    //     paramter: paramter,
    //     required: Some(""),
    // };
    // let pulse = GeminiPulse::new()
    //     .env("GEMINI_API_KEY")
    //     .model(gemini_ai::Models::GEMINI_1_0_PRO)
    //     .train(&response)
    //     .instruction("be good")
    //     .tell("tell the light to on")
    //     .build()
    //     .output();

    // let feature1 = Properties::new(
    //     "get_current_place_detail",
    //     "current palce details",
    //     Some(gemini_ai::pulse::format::Paramters {
    //         r#type: String::from("object"),
    //         properties: gemini_ai::pulse::format::SubProperties {
    //             name: String::from("events"),
    //             r#type: String::from("string"),
    //             description: String::from("Render all the events located in current location"),
    //         },
    //     }),
    //     Some(&["events"]),
    // );

    // let feature = feature(&[&feature1]);

    // println!("{}", pluse);
}
