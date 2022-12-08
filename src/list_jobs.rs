use {
    crate::*,
    termimad::{
        minimad::{
            OwningTemplateExpander,
            TextTemplate,
        },
        MadSkin,
    },
};

pub fn print_jobs(package_config: &PackageConfig) {
    static MD: &str = r#"
    |:-:|:-|
    |**job**|**command**|
    |:-:|:-|
    ${jobs
    |${job_name}|${job_command}|
    }
    |-|-|
    default job: ${default_job}
    "#;
    let mut expander = OwningTemplateExpander::new();
    let mut jobs: Vec<_> = package_config.jobs.iter().collect();
    jobs.sort_by_key(|(name, _)| name.to_string());
    for (name, job) in &jobs {
        expander.sub("jobs")
            .set("job_name", name)
            .set("job_command", job.command.join(" "));
    }
    expander.set("default_job", &package_config.default_job);
    let skin = MadSkin::default();
    skin.print_owning_expander(&expander, &TextTemplate::from(MD));
}

