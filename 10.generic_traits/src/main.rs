use crate::traits::{NewsArticle, Pair, SocialPost, Summary};

mod traits;

fn main() {
    let article_1 = NewsArticle {
        headline: String::from("headline1"),
        location: String::from("location1"),
        author: String::from("author1"),
        content: String::from("content1"),
    };
    println!("{}", article_1.summarize());

    let social_post_1 = SocialPost {
        username: String::from("username_1"),
        content: String::from("content_1"),
        reply: false,
        repost: false,
    };
    println!("{}", social_post_1.summarize());

    traits::notify(&article_1);
    traits::notify_trait_bound_syntax(&article_1);

    traits::notify(&social_post_1);
    traits::notify_trait_bound_syntax(&social_post_1);

    let pair_1 = Pair::new(1, 2);
    pair_1.cmp_display();

    let pair_2 = Pair::new("abc", "bcd");
    pair_2.cmp_display();

    let pair_3 = Pair::new('a', 'b');
    pair_3.cmp_display();

    let number_list_1 = vec![1, 3, 2, 5];
    println!("largest value: {}", traits::largest(&number_list_1));

    let char_list_1 = vec!['a', 'c', 'x', 'm'];
    println!("largest value: {}", traits::largest(&char_list_1));

    let string_list_1 = vec!["azvb", "cbca", "xzad", "mbca"];
    println!("largest value: {}", traits::largest(&string_list_1));
}
