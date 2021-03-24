extern crate maud;

use maud::{html, Markup};

pub fn getting_the_first_job() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        p{
            "i've been asked a few times if i had any tips and tricks on how to get a job as a recent bootcamp/uni grad. i figured it would be useful to write out a longer form thing that i could add to as time went on rather than giving out what i happen to remember at the time."
        }
        p{
            "probably the most useful thing i can say to do is to network. going to meetups, talking in your local slack group, anything that lets you make even small connections with people will help you stand out. my current job gets hundreds of applications for positions. most of them get filtered out by automated software. if you are able to talk to someone at the company, even if they aren't involved in the hiring process, it can at least get your resume out of that automatic filtering step."
        }
        p{
            "if you are like me and don't like to network, talking in your local slack/discord/chat group was a pretty low pressure, easy way to interact with people. people in my local slack group are also very willing to offer specific advice or critique resumes, which can be quite helpful. otherwise, attending meetups(once they're happening in person again) is a great way to talk to people. all of the developers i've met have been friendly and are more than willing to talk to beginners."
        }
        p{
            "finding freelance work is the second thing i would suggest doing. there are lots of places out there that need some sort of dev work. non-technical people who are trying to get their part-time startup off the ground can be a good source of work. small businesses or non-profits that need a website, or need changes made are another potential source of work. again, this often requires networking. there are websites that offer gig-type dev work, but they have seemed not very good when i looked at them. these jobs won't pay terribly well, but it will give you a product to show people and demonstrate that you are capable of developing a functioning product."
        }
        p{
            "contributing to open source can also be a good way to get your feet wet working with a larger product. most open source projects will have a " span.tag{"`getting started` "} "section that detail how to contribute to their project. there are also usually issues tagged: " span.tag{"`easy`"} ", " span.tag{"`beginner`"} ", or " span.tag{"`good-first-issue`"} " that are good issues to start with. finding an open source project you want to contribute to can be daunting. i would look at the projects you have used in the past and see if there are any in there that have things you would be interested in working on. github also has an open source search feature that you might use."
        }
        p{
            "the last thing i would recommend is to practice interviewing. interviews are stressful and it can make a huge difference if you are able to answer some of the more common interview questions without having to think too hard. you shouldn't be afraid of selling your skills. i wouldn't overstate your abilities, but i lost a lot of interviews that were otherwise going well because i downplayed my skills because i was insecure."
        }
        p{
            "it took me about 8 months to get my first full time job. it might take a while, but i believe in you."
        }
    };
    Ok(h)
}
