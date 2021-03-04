model.tags = [];
for content in model.contents {
    for tag in content.tags {
        if model.tags.some(|t|t == tag) {
            let group = model.tags.filter(|t|t.tag == tag )[0];
            group.contents.push(content);
        } else {
            let group = #{
                tag: tag,
                contents: [content]
            };
            model.tags.push(group);
        };
    }
}

model.archives = [];
for content  in model.contents {
    let date = content.create_time.split("T")[0];
    if model.archives.some(|a|a.date == date) {
        let group = model.archives.filter(|a|a.date == date)[0];
        group.contents.push(content);
    } else {
        let group = #{
            date: date,
            contents: [content]
        };
        model.archives.push(group);
    }
}

model.menus = [#{
    name: "Home",
    path: "/"
},#{
    name: "Tag",
    path: "/"
},#{
    name: "Archive",
    path: "/"
},#{
    name: "Github",
    path: "/"
}];

model
