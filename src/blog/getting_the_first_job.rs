extern crate maud;

use maud::{html, Markup};

pub fn getting_the_first_job() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"Getting the First Job"}
        p{
            "I've been asked a few times if I had any tips and tricks on how to get a job as a recent bootcamp/uni grad. I figured it would be useful to write out a longer form thing that I could add to as time went on rather than giving out what I happen to remember at the time."
        }
        p{
            "Probably the most useful thing I can say to do is to network. Going to Meetups, talking in your local Slack group, anything that lets you make even small connections with people will help you stand out. My current job gets hundreds of applications for positions. Most of them get filtered out by automated software. If you are able to talk to someone at the company, even if they aren't involved in the hiring process, it can at least get your resume out of that automatic filtering step."
        }
        p{
            "If you are like me and don't like to network, talking in your local Slack/Discord/chat group was a pretty low pressure, easy way to interact with people. People in my local slack group are also very willing to offer specific advice or critique resumes, which can be quite helpful. Otherwise, attending meetups(once they're happening in person again) is a great way to talk to people. All of the developers I've met have been friendly and are more than willing to talk to beginners."
        }
        p{
            "Finding freelance work is the second thing I would suggest doing. There are lots of places out there that need some sort of dev work. Non-technical people who are trying to get their part-time startup off the ground can be a good source of work. Small businesses or non-profits that need a website, or need changes made are another potential source of work. Again, this often requires networking. There are websites that offer gig-type dev work, but they have seemed not very good when I looked at them. These jobs won't pay terribly well, but it will give you a product to show people and demonstrate that you are capable of developing a functioning product."
        }
        p{
            "Contributing to open source can also be a good way to get your feet wet working with a larger product. Most open source projects will have a " span.tag{"`getting started` "} "section that detail how to contribute to their project. there are also usually issues tagged: " span.tag{"`easy`"} ", " span.tag{"`beginner`"} ", or " span.tag{"`good-first-issue`"} " that are good issues to start with. Finding an open source project you want to contribute to can be daunting. I would look at the projects you have used in the past and see if there are any in there that have things you would be interested in working on. Github also has an open source search feature that you might use."
        }
        p{
            "The last thing I would recommend is to practice interviewing. Interviews are stressful and it can make a huge difference if you are able to answer some of the more common interview questions without having to think too hard. You shouldn't be afraid of selling your skills. I wouldn't overstate your abilities, but I lost a lot of interviews that were otherwise going well because I downplayed my skills because I was insecure."
        }
        p{
            "It took me about 8 months to get my first full time job. It might take a while, but I believe in you."
        }
    };
    Ok(h)
}
