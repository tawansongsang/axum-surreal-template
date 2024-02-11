
USE NS ns_template;
USE DB db_template;
//UPDATE task:Ay5RJ87J5dSkChl9KlSD MERGE {
UPDATE type::thing("task", "Ay5RJ87J5dSkChl9KlSD") MERGE {
    title: 'task update demo1',
    done: true,
    update_by: user_info:iR1f8i7Wg7jipR3uhDhJ,
    update_on: time::now(),
};
