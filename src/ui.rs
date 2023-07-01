
pub(crate) struct Button<'a> {
    pub name: &'a str, // Size of pixel array
    pub x: u32,
    pub y: u32,
}

// every time the display input function detects a mouse click it calls this function
pub(crate) fn mouse_click(){
    //check to see if the click is on a button and if so send the name if the button to the ui distributor
}

pub(crate) fn ui_distributor(name: &str){
    match name {
        "test" => {
            //code that the button is supposed to run
        }
        _ => {
            
        }
    }
}