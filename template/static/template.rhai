model.tags = [];
model.fill_with(model.raw);

for content in model.contents {
    content.fill_with(content.raw);
}

for content in model.contents {
    for name in content.tags {
        if model.tags.some(|t|t.name == name) {
            let group = model.tags.filter(|t|t.name == name )[0];
            group.contents.push(content);
        } else {
            let group = #{
                name: name,
                contents: [content]
            };
            model.tags.push(group);
        };
    }
}

model.archives = [];
for content  in model.contents {
    let name = content.create_time.split("T")[0];
    if model.archives.some(|a|a.name == name) {
        let group = model.archives.filter(|a|a.name == name)[0];
        group.contents.push(content);
    } else {
        let group = #{
            name: name,
            contents: [content]
        };
        model.archives.push(group);
    }
}

model
