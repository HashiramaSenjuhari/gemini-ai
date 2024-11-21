use gemini_ai::{
    // cloud::cloud::check_state,
    decode_gemini,
    format::{key, nested, Function, Parameters, Properties, PropertiesParameter},
    pulse::pulse::GeminiPulse,
    schema::schema_store,
    GeminiContentGenBuilder,
    Pair,
};

fn main() {
    let value = key("name", "STRING");
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
        .kind(gemini_ai::Kind::Image("OIP.jpeg"))
        .instruction(
            "you are great image analyzer and tell the image design accuratly and how it can be made great",
        )
        .text("image")
        .build()
        .output();
    println!("{}", builder);
    // // let string = decode_gemini(&builder);
    // // println!("{:?}", string);
    // check_state();
    let paramter = PropertiesParameter::new("trun_on_light", "string", "turn_on_the_led");
    let paramterss = Properties::parameter("string", &paramter, Some(&["turn_on_the_led"]));
    let parameter1 = Parameters::parameter("turn_on_the_led", "billionaire", &paramterss);

    let paramter = PropertiesParameter::new("trun_on_light", "string", "trun_on_light");
    let paramterss = Properties::parameter("string", &paramter, Some(&["turn_on_light"]));
    let parameter2 = Parameters::parameter("turn_on_light", "billionaire", &paramterss);

    let paramter = PropertiesParameter::new("trun_on_light", "string", "turn_on_the_led");
    let paramterss = Properties::parameter("string", &paramter, Some(&["turn_on_light"]));
    let parameter3 = Parameters::parameter("turn_on_light", "billionaire", &paramterss);

    let response = Function::new(&[parameter1, parameter2, parameter3]);
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
}
