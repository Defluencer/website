use crate::components::Navbar;

use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

/// The get started page.
pub struct Start {}

impl Component for Start {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Navbar />
                <ybc::Section>
                    <ybc::Container>
                        <ybc::Title size=ybc::HeaderSize::Is5 >
                            { "Understanding the protocol" }
                        </ybc::Title>
                        <p>
                        {
                            "Building decentralized applications is very different from what you might used to.
                            To work effectively, mastering theses concepts is key."
                        }
                        </p>
                        <br />
                        <ul>
                            <li> { "- Content Addressing" } </li>
                            <li> { "- Merkle Directed Acyclic Graphs" } </li>
                            <li> { "- Content Identifiers" } </li>
                        </ul>
                        <br />
                        <p>
                        { "Thankfully, we have a " }
                            <a href="https://proto.school/course/ipfs"> { "ProtoSchool" } </a>
                        { " for that."}
                        </p>
                        <br />
                        <p>
                        {
                            "Next is "
                        }
                            <a href="https://ipld.io/docs/intro/hello-world/"> { "IPLD" } </a>
                        {
                            ", protocol specific "
                        }
                            <a href="https://github.com/Defluencer/rust-linked-data"> { "schemas" } </a>
                        { " and finally cryptography knowledge can also help." }
                        </p>
                    </ybc::Container>
                </ybc::Section>
                <ybc::Section>
                    <ybc::Container>
                        <ybc::Title size=ybc::HeaderSize::Is5 >
                            { "Implementation" }
                        </ybc::Title>
                        <p>
                        {
                            "The protocol can be implemented in any programming language and most often using existing code is best.
                            "
                        }
                        </p>
                        <br />
                        <ul>
                            <li><a href="https://github.com/Defluencer/rust-defluencer">{"rust-defluencer"}</a></li>
                            <li><a href="https://github.com/Defluencer/rust-linked-data">{"rust-linked-data"}</a></li>
                            <li><a href="https://github.com/Defluencer/website">{"Defluencer website"}</a></li>
                        </ul>
                        <br />
                        <p>
                            { "Visit the " }
                            <a href="https://github.com/Defluencer"> { "Defluencer" } </a>
                            { " project page on github for more." }
                        </p>
                    </ybc::Container>
                </ybc::Section>
                <ybc::Section>
                    <ybc::Container>
                        <ybc::Title size=ybc::HeaderSize::Is5 >
                            { "Example" }
                        </ybc::Title>
                        <p>
                        {
                            "This website is one implementation of the protocol.
                            Navigate to the setting page and following the instructions.
                            You will be asked to install IPFS and need to configure your node appropriately.
                            After it is done, the live and content feed pages will be fonctional."
                        }
                        </p>
                    </ybc::Container>
                </ybc::Section>
            </>
        }
    }
}
