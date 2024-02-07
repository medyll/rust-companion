
use cpal::traits::{DeviceTrait, HostTrait};
use iced::{executor, Application, Command, Element, Settings, theme::Theme };
use iced::widget::{pick_list, scrollable};
//use iced::widget::Text;

#[derive(Debug, Clone)]
enum Message {
    AudioDeviceChanged(Option<String>),
}

struct Companion {
    audio_input_device: Option<String>,
    audio_device_list: pick_list::State,
    audio_input_list: Vec<String>, 
    selected_audio_input_device: Option<Option<String>>,
}

impl Application for Companion {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    
    fn new(_flags: ()) -> (Companion, Command<Self::Message>) {
        // let audio_input_list = get_input_audio_list();
        (Companion{
            audio_input_device: None,
            audio_device_list:pick_list::State::default(),
            audio_input_list: get_input_audio_list(),    
            selected_audio_input_device: None,   
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Hello, world!")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::AudioDeviceChanged(device) => {
                self.audio_input_device = device;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        //let selected_audio_input_device = self.selected_audio_input_device.clone().unwrap_or(Option::<String>::from("No device selected"));
        /* let pick_list = pick_list(
            &mut self.audio_input_list, 
            Some(selected_audio_input_device),
            Message::AudioDeviceChanged,
        ) */
        /* let content = column![
            vertical_space(600),
            "Which is your favorite language?",
            "pick_list",
            vertical_space(600),
        ]; */
        
        scrollable("content").into() 
    }
} 

pub fn main() -> iced::Result {
    Companion::run(Settings::default())
}



fn get_input_audio_list() -> Vec<String> {
    let host = cpal::default_host();
    let input_devices = host.input_devices().unwrap();

    let mut device_names = Vec::new();
    for device in input_devices {
        device_names.push(device.name().unwrap());
    }

    device_names
}

fn get_output_audio_list() -> Vec<String> {
    let host = cpal::default_host();
    let input_devices = host.input_devices().unwrap();

    let mut device_names = Vec::new();
    for device in input_devices {
        device_names.push(device.name().unwrap());
    }

    device_names
}