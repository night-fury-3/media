#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::AudioBook {
        title: String::from("An Audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Moive"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    print_media(good_movie);
    print_media(bad_book);
    print_media(audiobook);
}
