use crate::{app::AppRoute, components::Navbar, utils::feature_card};

use yew::prelude::{classes, html, Component, ComponentLink, Html, ShouldRender};

/// The landing page.
pub struct Home {}

impl Component for Home {
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
        let body_one = html! {
            <ybc::Container classes=classes!("has-text-centered") >
                <ybc::Title tag="h1" >
                {
                    "Build social media applications and websites on the web 3.0"
                }
                </ybc::Title>
                 <br />
                <ybc::Subtitle tag="h2" >
                {
                    "Defluencer is a protocol for decentralized social media.
                    Are you an influencer? Build your own website to show the world your awesome content!
                    Do you love wombats? Build a platform and agregate the world's content on the subject!
                    With immutable and interoperable content, your data cannot be changed and will follow you to any app or website built on the protocol."
                }
                </ybc::Subtitle>
                <ybc::ButtonRouter<AppRoute> route=AppRoute::Start classes=classes!("is-primary") >
                    {"Get Started"}
                </ybc::ButtonRouter<AppRoute>>
            </ybc::Container>
        };

        let live_card = feature_card(
            "Live Streaming",
            "Adaptative bitrate, custom resolutions, qualities and codecs.",
        );

        let chat_card = feature_card(
            "Live Chat",
            "Choose a display name and use your Ethereum address as your identifier.",
        );

        let streaming_card = feature_card(
            "On Demand Streaming",
            "Host videos or save your live streams and chat for posterity.",
        );

        let blog_card = feature_card(
            "Blogs",
            "Micro blog posts or lengthy articles via markdown files.",
        );

        let feed_card = feature_card(
            "Content Feed",
            "Organize your content into a multimedia feed that people can follow.",
        );

        let comments_card = feature_card(
            "Commentary",
            "Comment on any media or read what people you follow have to say.",
        );

        html! {
            <>
                <Navbar />
                <ybc::Hero classes=classes!("is-medium") body=body_one />
                <ybc::Section>
                    <ybc::Container>
                        <ybc::Columns classes=classes!("is-multiline") >
                            { live_card }
                            { chat_card }
                            { streaming_card }
                            { blog_card }
                            { feed_card }
                            { comments_card }
                        </ybc::Columns>
                    </ybc::Container>
                </ybc::Section>
                <ybc::Section>
                    <ybc::Container>
                        <ybc::Title size=ybc::HeaderSize::Is5 >
                            { "How does it work?" }
                        </ybc::Title>
                        <p>
                            { "With " }
                            <a href="https://ipfs.io/"> { "IPFS" } </a>
                            { " & " }
                            <a href="https://ipld.io/"> { "IPLD" } </a>
                            {
                                " you get a network of content-addressable data that can be linked together in a web and
                                as a by-product immutability, which means your content cannot be changed, you can share it with your friends
                                and they will redistribute it, as you do for them.
                                The defluencer protocol include standard data formats for social media content and can be easily extended."
                            }
                        </p>
                        <br />
                        <ybc::Title size=ybc::HeaderSize::Is5 >
                            { "This is too good to be true! Is there a catch?" }
                        </ybc::Title>
                        <p>
                            {
                                "Yes! Since there's is no company, no server and no authority that freedom comes with responsabilities."
                            }
                            <ul>
                                <li>
                                {
                                    "First, AT LEAST one person on the ENTIRE network MUST be online with that content or it cannot be accessed.
                                    It doesn't have to be you, they don't need ALL your content but more the better, friends should help each others."
                                }
                                </li>
                                <li> { "Second, the network is PUBLIC, anyone can share your content even if you don't want to and what's on the internet is forever." } </li>
                                <li> { "Third, what happen in cyberspace is not separate from the real world, cyberspace IS the real world!" } </li>
                            </ul>
                        </p>
                    </ybc::Container>
                </ybc::Section>
                <ybc::Footer>
                    <ybc::Container>
                        <ybc::Columns>
                            <ybc::Column classes=classes!("is-half") >
                                <p>
                                    <a href="https://github.com/Defluencer/website">
                                        <span class="icon-text">
                                            <span> {"Source Code"} </span>
                                            <span class="icon"><i class="fab fa-github"></i></span>
                                        </span>
                                    </a>
                                </p>
                                <p>
                                    <a href="https://twitter.com/defluencermedia">
                                        <span class="icon-text">
                                            <span> {"Twitter"} </span>
                                            <span class="icon"><i class="fab fa-twitter"></i></span>
                                        </span>
                                    </a>
                                </p>
                            </ybc::Column>
                            <ybc::Column classes=classes!("is-half") >
                                <a href="https://bulma.io">
                                    <img src="https://bulma.io/images/made-with-bulma.png" alt="Made with Bulma" width="128" height="24" />
                                </a>
                            </ybc::Column>
                        </ybc::Columns>
                    </ybc::Container>
                </ybc::Footer>
            </>
        }
    }
}
