mod fluid;
use yew::{html, Component, Context, Html};
use gloo::timers::callback::Interval;
use fluid::Fluid;

pub enum Msg {
    Stop,
    Start,
    Tick,
}

pub struct App {
    state: bool,
    message: String,
    fl: Fluid,
    _interval: Interval
}

impl App {
    fn change_message(&mut self) {
        match self.state {
            true => { self.message = "Running".to_string() }
            false => { self.message = "Not running".to_string() }
        }
    }
    
    fn view_cell(&self, pression: f64) -> Html {
        let color_statement: String = format!("background-color: hsl({} 80% 50%);", pression);
        html! {
            <div class="cellule" style={color_statement}></div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(33, move || callback.emit(()));
        let fl = Fluid::new(50, 50);
        Self { state: false, message: "hello there".to_string(), fl, _interval: interval}
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Stop => { self.state = false; self.change_message(); true }
            Msg::Start => { self.state = true; self.fl.excitation(); self.change_message(); true }
            Msg::Tick => { 
                if self.state {
                    self.fl.calculate(); 
                    true
                }
                else { false }
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let cells_rows = {
            self.fl.p.rows.iter().enumerate().map(|(y, pression_row)|{
                let cells = pression_row.iter().enumerate().map(|(x, pression)|{
                    self.view_cell(pression.clone())
                });
                html! {
                    <div class="cells_row">
                        { for cells }
                    </div>
                }
            })
        };
        html! {
        <div>
            <button class="special_button" onclick={ctx.link().callback(|_| Msg::Start)}>{"Start"}</button>
            <button class="special_button" onclick={ctx.link().callback(|_| Msg::Stop)}>{"Stop"}</button>
            <section class="cells_area">
                <div class="cells_placeholder">
                    <div>
                        { for cells_rows }
                    </div>
                </div>
            </section>
            <p class="special_par">
                { &self.message }
            </p>
        </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
