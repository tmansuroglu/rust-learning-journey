trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl Summary for SocialPost {}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    let article = NewsArticle {
        headline: String::from("qwe"),
        location: String::from("asd"),
        author: String::from("sg"),
        content: String::from("of course, as you probably already know, people"),
    };

    println!("1 new social post: {}", post.summarize());

    println!("1 new news article: {}", article.summarize());

    // fn some_fn<T: Display + Clone, U: Clone + Debug>(t:&T,u:&U)-> i32 {
    //     0
    // }

    // fn some_function<T, U>(t:&T, u:&U) -> i32
    // where
    //     T: Dislay + Clone,
    //     U: Clone + Debug
    // {
    //     0
    // }

    fn returns_summarizable() -> impl Summary {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            repost: false,
        }
    }

    //  This annotation means an instance of 'ImportantExcerpt' can’t outlive the reference it holds in its 'part' field.
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}
