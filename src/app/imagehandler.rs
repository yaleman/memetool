use yew::prelude::*;


#[derive(Eq, PartialEq, Properties)]
pub struct ImageRenamerProps {
    pub original_path: String,
}

pub struct ImageRenamer {
    pub original_path: String,
    pub new_path: Option<String>,
}

pub enum ImageRenamerMsg {
    // Start,
    // Renaming,
    // Cancelled,
}

impl Component for ImageRenamer{
    type Message = ImageRenamerMsg;
    type Properties = ImageRenamerProps;

    fn create(ctx: &Context<Self>) -> Self {
        ImageRenamer {
            original_path: ctx.props().original_path.to_owned(),
            new_path: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let file_name = match &self.new_path {
            Some(new_path) => new_path.split("/").last().unwrap(),
            None => {
                self.original_path.as_str().split("/").last().unwrap()
            }
        };
        let file_name = file_name.to_string();

        let new_path = match self.new_path.clone() {
            Some(new_name) => html!{<p>{"to "}{new_name}</p>},
            None => html!{<></>}
        };


        html!{
            <>
            <form>
            <table cellpadding="3" cellspacing="0" width="100%">
                <tr>
                    <td class="col">{"Original path: "}</td>
                    <td class="col">{self.original_path.clone()}</td>
                </tr>
                <tr>
                    <td class="col">{"New Path: "}</td>
                    <td class="col min-width: 80%">
                        <input type="text" name="new_path" style="min-width: 80%; max-width: 100%" value={file_name} />
                        </td>
                </tr>
            </table>
            </form>
            {new_path}
            </>
        }
    }
}