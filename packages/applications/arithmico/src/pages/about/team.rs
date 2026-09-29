use leptos::prelude::*;
use toml::{Table, Value, map::Map};
use translate::FormattedMessage;

pub struct TeamMember {
    name: String,
    email: String,
    role: String,
}

fn get_team_member(table: &Map<String, Value>) -> Option<TeamMember> {
    let name = table.get("name")?.as_str()?.to_string();
    let email = table.get("email")?.as_str()?.to_string();
    let role = table.get("role")?.as_str()?.to_string();
    Some(TeamMember { name, email, role })
}

fn get_team_members() -> Option<Vec<TeamMember>> {
    let data = include_str!("../../../../../../team.toml");
    let data = data.parse::<Table>().ok()?;
    let members = data.get("members")?.as_array()?;

    let mut team_members = Vec::new();
    for member in members {
        let Some(member) = member.as_table() else {
            continue;
        };
        let Some(team_member) = get_team_member(member) else {
            continue;
        };
        team_members.push(team_member);
    }

    Some(team_members)
}

#[component]
pub fn TeamMembers() -> impl IntoView {
    let team_members = get_team_members().unwrap_or_default();

    view! {
        <section>
            <h2>
                <FormattedMessage id="about.team" />
            </h2>
            <ul class="team-list">
                {team_members
                    .into_iter()
                    .map(|member| {
                        view! {
                            <li class="team-member">
                                <p class="name">{member.name.clone()}</p>

                                <dl>
                                    <dd>
                                        <FormattedMessage id="about.team.member.role" />
                                    </dd>
                                    <dt>{member.role.clone()}</dt>

                                    <dd>
                                        <FormattedMessage id="about.team.member.email" />
                                    </dd>
                                    <dt>
                                        <a href=format!(
                                            "mailto:{}",
                                            member.email.clone(),
                                        )>{member.email.clone()}</a>

                                    </dt>
                                </dl>
                            </li>
                        }
                    })
                    .collect_view()}

            </ul>
        </section>
    }
}
