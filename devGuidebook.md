Written by Dave Meredith, originally for the STFC Hartree Centre. 
Last Updated Oct 24.  Enjoy. 

```dataviewjs
dv.view('toc')
```

1. [Table of contents](#Table-of-contents)
2. [Why Software Dev Guidebook](#Why-Software-Dev-Guidebook)
    1. [Target Audience](#Target-Audience)
    2. [If it is not in Git it does not exist](#If-it-is-not-in-Git-it-does-not-exist)
        1. [Feature Branching vs Trunk Based development](#Feature-Branching-vs-Trunk-Based-development)
        2. [Integrating Upstream Changes](#Integrating-Upstream-Changes)
        3. [Rebase vs Merge](#Rebase-vs-Merge)
    3. [Adopt Semantic Versioning for tags and releases](#Adopt-Semantic-Versioning-for-tags-and-releases)
    4. [Review each others code and be supportive](#Review-each-others-code-and-be-supportive)
    5. [If the critical path has not been reviewed it should not go onto master branch](#If-the-critical-path-has-not-been-reviewed-it-should-not-go-onto-master-branch)
    6. [Continuous Integration - If it does not have tests it does not work](#Continuous-Integration---If-it-does-not-have-tests-it-does-not-work)
    7. [Continuous Delivery](#Continuous-Delivery)
    8. [Continuous Deployment](#Continuous-Deployment)
    9. [If it Does not have Documentation it is Not Usable](#If-it-Does-not-have-Documentation-it-is-Not-Usable)
    10. [Learning mindset](#Learning-mindset)
    11. [Customer Bill of Rights (modified from Uncle Bob Martin - Clean Agile)](#Customer-Bill-of-Rights-(modified-from-Uncle-Bob-Martin---Clean-Agile))
    12. [Developer Bill of Rights (modified from Uncle Bob Martin - Clean Agile)](#Developer-Bill-of-Rights-(modified-from-Uncle-Bob-Martin---Clean-Agile))
    13. [Tooling](#Tooling)
        1. [Do not be Smart - Use the Right Tools for the Job and for your customer](#Do-not-be-Smart---Use-the-Right-Tools-for-the-Job-and-for-your-customer)
        2. [To Garbage Collect or Not To GC](#To-Garbage-Collect-or-Not-To-GC)
        3. [Kanban, Jira & Confluence](#Kanban,-Jira-&-Confluence)
        4. [Gitlab and Version Control](#Gitlab-and-Version-Control)
        5. [Container Repository](#Container-Repository)
        6. [Build Tools](#Build-Tools)
        7. [Static Code Analysis](#Static-Code-Analysis)
        8. [Containerisation for Portability](#Containerisation-for-Portability)
        9. [Workflows and Containerisation](#Workflows-and-Containerisation)
3. [Development Guide Best Practices](#Development-Guide-Best-Practices)
    1. [Quality is the best shortcut - Fowler Design Stamina Hypothesis](#Quality-is-the-best-shortcut---Fowler-Design-Stamina-Hypothesis)
    2. [Naming with Meaningful and Descriptive Names](#Naming-with-Meaningful-and-Descriptive-Names)
    3. [Testing has Three Main Purposes](#Testing-has-Three-Main-Purposes)
    4. [Keep Classes and Functions Smallish](#Keep-Classes-and-Functions-Smallish)
    5. [Limit the Number of Function Arguments](#Limit-the-Number-of-Function-Arguments)
    6. [Functions should do one thing and do it well](#Functions-should-do-one-thing-and-do-it-well)
    7. [Classes and Code Should be Cohesive](#Classes-and-Code-Should-be-Cohesive)
    8. [Classes should have only one reason to change and do one thing and do it well](#Classes-should-have-only-one-reason-to-change-and-do-one-thing-and-do-it-well)
    9. [If using OOP know SOLID](#If-using-OOP-know-SOLID)
    10. [Dependency Inversion Principle](#Dependency-Inversion-Principle)
    11. [Dependency Injection and Inversion of Control (IoC)](#Dependency-Injection-and-Inversion-of-Control-(IoC))
    12. [Avoid Inheritance Tax - Prefer Object Composition, Interfaces, Traits, Mixins to achieve Ad-hoc Polymorphism](#Avoid-Inheritance-Tax---Prefer-Object-Composition,-Interfaces,-Traits,-Mixins-to-achieve-Ad-hoc-Polymorphism)
    13. [It Should Not be Possible to Create an Object in an Invalid State](#It-Should-Not-be-Possible-to-Create-an-Object-in-an-Invalid-State)
    14. [Know Some Design Patterns](#Know-Some-Design-Patterns)
        1. [The Strategy Pattern Example](#The-Strategy-Pattern-Example)
        2. [The Visitor Pattern](#The-Visitor-Pattern)
    15. [Consider using the Builder Pattern for More Complex Object Creation Scenarios](#Consider-using-the-Builder-Pattern-for-More-Complex-Object-Creation-Scenarios)
    16. [Information Hiding](#Information-Hiding)
    17. [DRY – Do not Repeat Yourself](#DRY-%E2%80%93-Do-not-Repeat-Yourself)
    18. [YAGNI – You Are not Going to Need It](#YAGNI-%E2%80%93-You-Are-not-Going-to-Need-It)
    19. [Comment in-line As You Go](#Comment-in-line-As-You-Go)
    20. [The Boy Scout Rule](#The-Boy-Scout-Rule)
    21. [Law of Demiter and Train Wrecks](#Law-of-Demiter-and-Train-Wrecks)
    22. [Functional vs OOP - Choose Two](#Functional-vs-OOP---Choose-Two)
    23. [Do not pollute Functional Code with Mutable State](#Do-not-pollute-Functional-Code-with-Mutable-State)
    24. [Make Immutability your Default](#Make-Immutability-your-Default)
    25. [Interior Mutability](#Interior-Mutability)
    26. [Use Calculations Where Possible to Limit Side Effects](#Use-Calculations-Where-Possible-to-Limit-Side-Effects)
    27. [Separate Operations from Calculations](#Separate-Operations-from-Calculations)
    28. [Error Handling - 4 Types of Problems](#Error-Handling---4-Types-of-Problems)
    29. [Error Handling - Exceptions vs Errors-as-Values](#Error-Handling---Exceptions-vs-Errors-as-Values)
    30. [Error Handling - Exceptions should not be used for flow control, use errors-as-values instead, exceptional != conditional](#Error-Handling---Exceptions-should-not-be-used-for-flow-control,-use-errors-as-values-instead,-exceptional-!=-conditional)
    31. [Error Handling - Only use exceptions for exceptional situations such as coding errors and unexpected situations, exceptional != conditional](#Error-Handling---Only-use-exceptions-for-exceptional-situations-such-as-coding-errors-and-unexpected-situations,-exceptional-!=-conditional)
    32. [Error Handling - Provide Relevant Exceptions for the abstraction layer](#Error-Handling---Provide-Relevant-Exceptions-for-the-abstraction-layer)
    33. [Error Handling - Bubble Exceptions Upwards or Trap at Source](#Error-Handling---Bubble-Exceptions-Upwards-or-Trap-at-Source)
    34. [Error Handling – Model the Absence of Value Explicitly](#Error-Handling-%E2%80%93-Model-the-Absence-of-Value-Explicitly)
    35. [Error Handling in Functional Programming – Error Monads such as Either & Validated](#Error-Handling-in-Functional-Programming-%E2%80%93-Error-Monads-such-as-Either-&-Validated)
    36. [Data Orientated Programming with Algebraic Data Types (ADTs)](#Data-Orientated-Programming-with-Algebraic-Data-Types-(ADTs))
    37. [Concurrency & Parallelism](#Concurrency-&-Parallelism)
        1. [Know the difference between IO bound tasks and CPU bound tasks and their common solution patterns](#Know-the-difference-between-IO-bound-tasks-and-CPU-bound-tasks-and-their-common-solution-patterns)
    38. [Security Development Practices](#Security-Development-Practices)
4. [Agile Process Guide a.k.a Feedback Driven Development](#Agile-Process-Guide-a.k.a-Feedback-Driven-Development)
    1. [Design Thinking Workshops and Scoping Document](#Design-Thinking-Workshops-and-Scoping-Document)
    2. [Epics and Work Package Span Multiple Sprints](#Epics-and-Work-Package-Span-Multiple-Sprints)
    3. [Define user stories either with the INVEST Framework, Who-What-Why or the Connextra Card Template – all are good and you don’t need to be too rigid](#Define-user-stories-either-with-the-INVEST-Framework,-Who-What-Why-or-the-Connextra-Card-Template-%E2%80%93-all-are-good-and-you-don%E2%80%99t-need-to-be-too-rigid)
    4. [Arrange core user stories into a Journey Map with a narrative flow or backbone of Big Activities moving from left to right](#Arrange-core-user-stories-into-a-Journey-Map-with-a-narrative-flow-or-backbone-of-Big-Activities-moving-from-left-to-right)
    5. [Task Backlog](#Task-Backlog)
    6. [Requirements Document and System Architecture Document](#Requirements-Document-and-System-Architecture-Document)
    7. [1-to-2-week Sprints](#1-to-2-week-Sprints)
    8. [Inline Testing](#Inline-Testing)
    9. [Demo and Playbacks](#Demo-and-Playbacks)
    10. [Acceptance, Sign Off and Cucumbers](#Acceptance,-Sign-Off-and-Cucumbers)
    11. [Iteration and Incrementalism](#Iteration-and-Incrementalism)
    12. [Cup Cake Roadmaps](#Cup-Cake-Roadmaps)
5. [Appendix - Recommended Texts](#Appendix---Recommended-Texts)
## Table of contents
## Why Software Dev Guidebook

To help everyone in the Centre build great software, we’ve put together a collection of development guidelines to help you build scalable, maintainable, reliable, performant, and usable code. Like all guidelines, these aren’t strict rules, and knowing when and where to apply these guidelines largely comes down to practice and experience.  This is not an exhaustive list. For more in-depth analysis, please see the list of recommended texts in the appendix. We’re not going to repeat all that good advice here, that’s what the books are for, but we’ve tried to distil a range of key recommendations.

Recognise that code is navigated and read far more than it is written, and that code is a form of expression designed for humans (machine code is for the machines).

_“Programs must be written for people to read, and only incidentally for machines to execute.__"_
Harold Abelson, the author of Structure and Interpretation of Computer Programs

Good quality code should read well, with details abstracted so that higher-level code reads almost like a form of ‘self-documenting’ story which is expressive of its intent. Implementation details should be hidden behind well named abstractions – a concept that is appropriate at a variety of scales, from variables at the small scale to names, functions, classes, interfaces, traits, facades, and packages/modules all the way up to application tiers and 'bounded contexts' (front-end, business logic, data tier), micro-services, services, to monoliths.

### Target Audience

This guide is intended for folks who read and write code. However, it is not possible to produce a ‘one size fits all’ set of guidelines for everyone.

If you predominantly use Python/R via Jupyter Notebooks for example, much of this advice might be overkill, and for that reason, there is separate section for notebooks. Recognise there are ways to bring more good software practices into Notebooks, see [https://nbdev.fast.ai/](https://nbdev.fast.ai/) which includes good stuff such as Git-friendly notebooks, built in support for CI/CD, support for tests as regular notebook cells and more.

Similarly, if you’re using low-level C/C++ or Fortran, many of the guidelines might simply be unavailable to you. Please bear this in mind, these are not rules, interpret them judiciously for your scenario, and as ever, the real answer is always “it depends”. We’ll keep evolving this document and welcome any comments.


### If it is not in Git it does not exist
- Use a GitLab/Github service - [https://gitlab.stfc.ac.uk/](https://gitlab.stfc.ac.uk/)
- Learn git concepts, not commands
- Branch early, commit little and often with ‘logically sensible commits’ multiple times a day.
- Use a dev branch for your main development and a main branch for your production releasable code. 
- Use topic branches (aka feature branches) for your new developments.

#### Feature Branching vs Trunk Based development

Research by Forsgen & Humble (‘Accelerate’ book) shows that long-lived feature branches that remain open for prolonged periods of time hinder delivery and productivity. Team members are less likely to interact and merge conflicts are more likely. The general recommendation is to try and merge feature branches into dev every one or two days. However, the Accelerate book authors do acknowledge that longer lived feature branches are suitable for open-source development where committers are less likely to work full time on features, and so often need more long-lived feature branches.

![](attachments/Pasted%20image%2020240610204547.png)
[https://nvie.com/posts/a-successful-git-branching-model/](https://nvie.com/posts/a-successful-git-branching-model/)

Tutorials: [https://github.com/davidmeredith/scdIntroToGit](https://github.com/davidmeredith/scdIntroToGit)

[1] [https://github.com/davidmeredith/scdIntroToGit/blob/master/introToGit.pdf](https://github.com/davidmeredith/scdIntroToGit/blob/master/introToGit.pdf)

#### Integrating Upstream Changes

There are two strategies to incorporating upstream commits from other branches - merging and rebasing.  Upstream commits are new commits that exist on another branch which need to be incorporated into your current branch to keep the branch up to date:

 a) Periodically merge the changes in the target branch (dev) into your feature branch. This creates the ‘braided’ graph pattern show opposite (flow is from top to bottom). When you’re ready to merge dev into master, a new merge-commit is created on the tip of the master branch. 
 ![](attachments/Pasted%20image%2020240610204748.png)
 
b) As shown below, rebasing basically ‘breaks off the feature branch from its root (yellow), and re-attaches it to the tip of the target branch (grey)’. During the rebase, the commits that exist on the feature branch are internally used to create a set of diffs in temporary files which are used by git to create new updated versions of your feature commits. Git needs to do this in order to incorporate any upstream changes that may have occurred on the target branch.
![](attachments/Pasted%20image%2020240610204801.png)

Note that rebasing doesn’t delete the feature branch, the feature branch still exists, but it is now ahead of the target branch, by 2 commits in the diagram below. To bring the target branch up to date, a fast-forward merge is required on the target branch.
![](attachments/Pasted%20image%2020240610204811.png)

#### Rebase vs Merge

Whether to rebase or merge is generally down to preference:

- Use ‘rebase’ to produce a clean and linear commit history. You can optionally use ‘interactive-rebasing’ and ‘squashing’ to clean up your commits too. Also observe the golden rule of rebasing: Don’t use rebasing if your feature branch is shared amongst multiple developers – rebasing essentially ‘pulls the rug from the under the feet of the other developers’ working on the feature branch.
- Use merge if you need to preserve a full history, for detailed auditing purposes for example. If this is the case, you probably don’t want to delete the feature branch after merging it, but you can do this later if needed.

### Adopt Semantic Versioning for tags and releases

- SemVer - [https://semver.org/](https://semver.org/)

### Review each others code and be supportive

- Foster a friendly and supportive environment and politely shout-out vulnerabilities and apparent issues, don’t be shy. As a reviewer you’ll learn something. Code reviews and pair-programming really does improve code quality and exposure to different projects/codes/practices.

### If the critical path has not been reviewed it should not go onto master branch

- Basic code quality relies on having at least two pairs of eyes on code, to catch errors, suggest improvements, build shared knowledge, and improve code style.  Get into the habit of developing on branches or forks and using pull / merge requests to facilitate code review before merging.

### Continuous Integration - If it does not have tests it does not work

- To be able to say something 'works' we judge it against some (implicit) criteria.  Writing tests makes our success criteria explicit.  Automation (Continuous Integration) prevents regressions.  Eventually, this leads towards test-driven development (TDD) where we think clearly about specifying what 'working' looks like up-front by writing code from a caller’s perspective.
- Test as you go along to a level that’s feasible and pragmatic. Extensive testing with production-level coverage (70 to 80%) is not always achievable or useful.  Given project budgets and timescales, focus on testing the application’s critical path as a minimum.
- Avoid gold-plating and focus on shipping code ASAP for customer review & feedback.
- Your Unit tests should be fast to complete – order of seconds. 
- Do not just rely on unit tests - system and integration tests are also needed. Unit tests alone will instil a false sense of security.
- Integration and System tests can take longer to complete.
- For more details on best practices for testing, including the different types of testing from Unit, Integration to System tests, see: [https://epubs.stfc.ac.uk/work/50305274](https://epubs.stfc.ac.uk/work/50305274)

### Continuous Delivery

Means that the software should always be in a releasable-ready condition.  This is a recommendation for your master and dev branches. If you’re run over by a bus (ROBAB), and someone must come along and pick up your code and they must fight with it from the outset, there’s a strong chance it will become shelfware.  If you need to have prolonged branches for experimentation that aren’t release-ready, create a feature branch such as ‘feature:homersSandbox’ to isolate your experiments. [https://epubs.stfc.ac.uk/work/47984368](https://epubs.stfc.ac.uk/work/47984368)

### Continuous Deployment

Means that once the software is merged into master branch, it automatically gets pushed into production. The idea is to make the large and risky ‘big feature release’ a legacy practice.  By continuously deploying to production with small and frequent updates, if something goes wrong, its quick and easy to rollback.  This might be a stretch for Hartree because its more relevant for long-lived production software & products, not so appropriate for proof of concepts.

### If it Does not have Documentation it is Not Usable

- Publish docs: Jira (task/sprints), Confluent (docs), GitLab, (code, merge requests, CI/CD), Bid register (PMO tracking tools)
- Use xDoc style code comments such as JavaDoc, PyDoc
- Document the intended purpose / intent of a function/class/package.
- If it’s tricky to document the intended purpose, then your class/function is likely too long and needs breaking down into smaller units.
- Always add a README.md.
- Always document inline - we _rarely_ go back and document our code after its written, fact.  
- End-users need to know how to use our software, so think about the right level of documentation for users. Consider separate user & developer docs. 
- The C4 approach to technical diagrams is good. [https://c4model.com/](https://c4model.com/)
- For more details on how to write good documentation for different users including ‘How To’ Guides, tools such as Markdown/AsciiDoctor, and reference guide formats, see: [https://epubs.stfc.ac.uk/work/47984356](https://epubs.stfc.ac.uk/work/47984356)

### Learning mindset

- Keep reading & learning and record all your training activities.
- Push beyond your comfort zone.
- Participate in RSE Skills & Learn sessions.
- Keep up the pursuit of software engineering craftsmanship, mastery and professionalism.
- Tinker - its really important to do hobby projects and dev stuff you enjoy.


### Customer Bill of Rights (modified from Uncle Bob Martin - Clean Agile)

Customers have the right to:
- An overall plan and to understand what can approximately be accomplished and at an estimated cost.
- Get the most possible value out of their projects.
- See progress in the development of a system.
- Change their mind, to substitute functionality, and to change priorities subject to agreement and re-scoping of the plan.
- Be informed of schedule and estimate changes, in time to choose how to reduce the scope to a meet a required date.
- Cancel at any time and be left with useful outputs reflecting their investment to date.

### Developer Bill of Rights (modified from Uncle Bob Martin - Clean Agile)

- Developers have the right to:
- Know what is needed with clear declarations of priority.
- To always produce high-quality work.
- Ask for and receive help from peers, managers, and customers.
- Make and update their estimates at any time.
- Challenge the task and the responsibilities instead of having them assigned – professionals accept work, they are not assigned work. A professional developer has every right to say no to a particular job for various reasons, from ethical to overloading.
- Knowing ‘accepting work’ comes with a cost – acceptance comes with responsibility.  

### Tooling

#### Do not be Smart - Use the Right Tools for the Job and for your customer    

As a centre, we should be using the right tools for the job, we all have our preferences, but there’s no need to be stubbornly loyal about a particular language or OS. As software professionals, we should recognise the right tools for the job and for our clients.  

#### To Garbage Collect or Not To GC

For HPC and when squeezing software into tight spaces such as in low-level systems programming (systems software isn’t HPC BTW), a Garbage Collected (GC) language probably isn’t the best choice. The GC adds lots of memory requirement. However, for full-stack, enterprise-applications / services, mobile, and general-purpose programming, it's probably best to use a memory safe language – “A human garbage collector is just wasted effort” (Eckle & Ward, Happy Path Programming). Similarly, there is a recognised shift in industry away from memory unsafe languages as it is widely known that the majority Common Vulnerability Exploits (CVEs) stem from unsafe memory language exploits, causing organisations such as [Google](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html) (for Android), [NSA and Microsoft to urge the use of memory-safe languages](https://www.theregister.com/2022/11/11/nsa_urges_orgs_to_use/).


Have the customer in mind. For example, Haskell and other Lisps are great (I’ve played with Clojure), but don’t be smart and use this as an opportunity to explore your favourite pet-programmer project, it’s not going to be much use to the customer. It’s hard to hire Haskell programmers.

![](attachments/Pasted%20image%2020240611091943.png)

#### Kanban, Jira & Confluence

Keep all project documents and the Decision Point Review templates in a single repo such as Confluence so the client has full visibility. Its best if the client creates the (free) Jira instance and invites us as admins on their Kanban/Jira/Confluence. This way, the client has ownership of the project after the project completes. Make sure you export and download a copy of the Jira archive file so we can restore it within our own Jira instances if needed in future or for follow on projects.

![](attachments/Pasted%20image%2020240611092007.png)

#### Gitlab and Version Control

For Hartree folks: Use the STFC Gitlab instance unless there is good reason not to: [https://gitlab.stfc.ac.uk/](https://gitlab.stfc.ac.uk/)

#### Container Repository

For Hartree folks: Use the STFC Harbor repository to upload and store containers: [https://harbor.stfc.ac.uk/](https://harbor.stfc.ac.uk/)

#### Build Tools

It is the developer’s responsibility to use an appropriate build tool that manages the dependencies of your project. It should be possible to clean the project, download dependencies, re-build a project, run unit and integration tests, and build a deployable package all from the command line. It is also the responsibility of developer to create the necessary environment configuration files (with specific version of modules or libraries) in a consistent state so that someone else can pick up your project easily.

#### Static Code Analysis

Static code analysis helps bring consistency to your code. Within a project, adopt the same style guides and agree the linters up front e.g., Black for Python, Google style guide for Java are good examples. There’s no ruling here, pick one that suits the team and be consistent. There are plenty of static code analysis tools out there such as CheckStyle, FindBugs, SonarCube, Black, Google linter, IDE checks.

#### Containerisation for Portability

Containers have become the de-facto way to mitigate the common claim “well, it works on my machine”. OCI compliant containers (Open Container Initiative) are great for wrapping code with all their dependencies into shareable images that can be uploaded to image repositories such as STFC’s Harbor service ([https://harbor.stfc.ac.uk/](https://harbor.stfc.ac.uk/)).  Containers are a great way to share code with your clients, especially if they need to run your code on their runtime platform. Containers are ubiquitous and can run on the Desktop, in Kubernetes clusters, in cloud Functions such as AWS Lambda, on HPC such as [Apptainer/Singularity](https://apptainer.org/), and more.

Here are some recommendations: 

- Only include runtime dependencies: Be mindful of what you’re including in your container - for production, you really don’t need to include compilers, package managers, and tools that are meant for use only at compile time (unless you’re containerising your dev environment of course). For example, [Google’s Distroless](https://github.com/GoogleContainerTools/distroless) containers and [Alpine Linux](https://alpinelinux.org/) are great for production use, providing cut down versions of Linux.  Containers such as these are great because they have less memory-footprint, and by reducing the amount of unnecessary stuff in them, they reduce the vulnerability attack surface, so they’re safer. [Here’s a great video](https://www.youtube.com/watch?v=6wYrAtngIVo) that shows how to build super slim production containers – ignore the fact that it is for Java, the concepts and tools discussed are generic and apply for many languages. 

- Don’t statically link glibc – use the [musl](https://musl.libc.org/) library instead. Glibc is notoriously unfriendly for containerisation and was not designed to be so.

- Don’t run your code within the container using the root user unless you really must.

- Use image layering to split up build time and run time dependencies. For example, having a separate 'build' and ‘run’ layers in your docker file allows you to copy only the built application code and dependencies into the container, leaving out all the unnecessary compile time dependencies. It also means you can re-run the container more quickly without having to re-build each time as build layers are built and cached locally.

- Testcontainers is awesome, download and run containerised apps/dependencies such as databases, services, tools and use them in your integration-test suites. Comes highly recommended: [https://www.testcontainers.org/](https://www.testcontainers.org/)

- See this great guide from the RSE team [for more info](https://softwareoutlook.ac.uk/best-practices-in-software-engineering/) on how to run HPC Singularity (now [Apptainer](https://apptainer.org/)) and Conda images with worked examples.

#### Workflows and Containerisation

Please refer to this separate document that characterises all the different types of workflows we use at Hartree, including Data Flow Engines that orchestrate containers using DAGs (Directed Acyclic Graphs): [http://purl.org/net/epubs/work/50844906](http://purl.org/net/epubs/work/50844906). Our ‘[Demystifying Data Engineering](https://www.hartree.stfc.ac.uk/events/demystifying-data-engineering/)’ Explain course provides more details into Data Flow runtimes and tooling.


## Development Guide Best Practices

This is not an exhaustive list of coding recommendations. For more in-depth explanations, please see the list of highly recommended texts. I’m not going to repeat all that excellent advice here, that’s what the books are for, but please find below a collection of development best practices that we should consider when developing our software. Like all guidelines, they aren’t strict rules, knowing when and where to judiciously apply largely comes down to experience. 

### Quality is the best shortcut - Fowler Design Stamina Hypothesis

- Think carefully about compromising the quality of your code for delivery speed; high quality code quickly becomes easier and faster to develop and overtakes hastily hacked together code. This is evidence based, see Martin Fowler's [Design Stamina Hypothesis]([https://martinfowler.com/bliki/DesignStaminaHypothesis.html](https://martinfowler.com/bliki/DesignStaminaHypothesis.html)) which describes how the velocity of software development declines with time due to poor design. We’ve also experienced this before in actual projects – the hypothesis has played out in practice at Hartree.

![](attachments/Pasted%20image%2020240611092227.png)

### Naming with Meaningful and Descriptive Names

 - “There are only two hard things in computer science, invalidation and naming things” (P.Karlton), see: [https://martinfowler.com/bliki/TwoHardThings.html](https://martinfowler.com/bliki/TwoHardThings.html)
 - Use intention revealing names for your classes, functions & variables. Don’t use single char var names (implicit loops etc are ok, but for global/module/class members, please use sensible names).  For example, ‘process_model’ is too generic, what does it mean? ‘execute_nlp_training_model’ is better, its more self-documenting.
 - Class names should have noun phrases e.g., Customer, WikiPage, Account, AccountParser.
 - Don’t use a comment when you can use a well named function or variable:
  

---




![](attachments/Pasted%20image%2020240611092516.png)

### Testing has Three Main Purposes

a)     **To assert the correctness of your code**. Sometimes you might counter “well, it’s not always possible to know the result of a calculation to assert because the result is non-deterministic”.  In this scenario, the following reasons to test still hold true!

b)     **To prevent regressions.** It really does build confidence in your code if you can quickly run a test suite to make sure you haven’t unexpectedly broken something.

c)     **To encourage good design** - it _really_ does actually - there’s lots of supporting research that shows this. When you write tests, you put yourself in the calling-code perspective, so you really do think about design such as loose coupling, separation of concerns, modularity, simplicity of API, and so on. If your code is hard to test, it’s likely too strongly coupled, and here Dependency Injection can help you.  Test Driven Development (TDD) is the ultimate testing practice, where you write tests first using an imaginary API, and then you fill in the details.

### Keep Classes and Functions Smallish

General rule – not much bigger than your screen’s viewport. I don't subscribe to certain author's view that functions should be no longer than four or five lines myself - I find it too difficult to hop around the code when functions are this small.   However, a viewport size / page is fine, and if you still need convincing, know that compilers can apply far more effective in-lining optimisations with smaller classes and functions.

### Limit the Number of Function Arguments

Consider using immutable Data Transfer Objects (aka DTOs / Data Objects / Records) if your function has more than ~4 arguments.

### Functions should do one thing and do it well

Separation of concerns at the function level.

### Classes and Code Should be Cohesive

Classes should be cohesive – high cohesion means the methods and variables of the class are co-dependent and often change together. This can be paraphrased as "Changes to the code over here should not affect code over there" and/or "Code that changes together stays together. " 

Here’s the authoritative view from the famous [Kent Beck from Nov (2022) and his ‘Tidy First’ approach to software development:](https://twitter.com/KentBeck/status/1587825849755049984)

![](attachments/Pasted%20image%2020240611092926.png)

Examples of patterns that support cohesion include the State Pattern. 
- In the State pattern, consider code that is widespread across many files that has exhaustive 'switch' or 'when' statements that reference a centrally declared enum set. The exhaustive switch/when statements execute different behaviours based on the current enum state value. If you add or remove a state enum option, you will need to update the exhaustive switch/when statements spread across your code-base. This is not a problem for small projects, but for large code bases it can require significant refactoring. The state pattern co-locates the state enum values with the dependent behaviour i.e., "things that change together, stay together." As an example, the central enum set could be replaced with a corresponding set of state objects, where each state object collects and implements the relevant state-dependent behaviour itself. There are several ways to implement this depending on you language of choice.  

### Classes should have only one reason to change and do one thing and do it well

- Separation of concerns at the class level.

### If using OOP know SOLID

OOP isn’t as fashionable as Functional these days, mostly because of the mantra 'prefer composition over inheritance'. However, I believe that most of the OOP principles are tried, tested and proven. You can’t argue against the principles of abstraction, information hiding and runtime polymorphism, these are good design principles even in Functional paradigms.  Arguably, what pure OOP does over-emphasise is deep & brittle inheritance hierarchies – you can favour composition instead in the right scenarios.

- Single Responsibility Principle
- Open for extension, closed for modification (dynamic dispatch and runtime polymorphism)
- Liskov’s Substitution principle (Barbara Liskov)
- Interface Segregation principle
- Dependency Inversion

### Dependency Inversion Principle

See the next two slides below. If this does not make sense, please refer the texts above, but in short it says, ‘don’t depend on implementation details, depend on abstractions between boundaries in your code’.

![](attachments/Pasted%20image%2020240611093615.png)

![](attachments/Pasted%20image%2020240611093638.png)

### Dependency Injection and Inversion of Control (IoC)

Basically, this means that your application code doesn’t itself create instances of classes/objects (dependencies), instead these objects are created separately and are then passed into your application code through a class constructors or setter methods. This ‘wiring together’ of dependencies can be done manually using the Factory Pattern or using an Inversion of Control container (I’m a fan of [Spring.io](https://stfc365.sharepoint.com/sites/HartreeRSEPublic/Shared%20Documents/General/spring.io)). For example, if Class A depends on class B, the IoC container will manage the lifecycle of class B for you (i.e., creation/destruction) and will inject class B into class A when needed. For required dependencies, meaning that class A can’t be initially created without a valid instance of class B, use ‘Constructor Injection’.  For optional dependencies, use setter injection.   The lifetime of a class is usually either singleton-scoped a.k.a., ‘application scoped’, where a single instance is created and managed (which means it must be thread-safe), or ‘prototype-scoped,’ where a new instance is always created & injected. There are several other special lifetimes such as ‘session-scope’ depending on your requirement.

So, what’s the benefit?  DI is likely overkill for small projects and scripts, but for larger more complex projects, it really is an excellent approach for decreasing _code coupling_ using abstractions such as interface implementations. It also enables far more effective testing - you can mock different dependencies/classes and inject them into your objects as needed to test different scenarios. Here’s a very simple example from Dave Farley’s ‘Modern Software Engineering’:

![](attachments/Pasted%20image%2020240611093717.png)

For the Car class, unless we decide to break encapsulation and make engine public, you can’t test the engine separately.  Our BetterCar allows you to mock or create a different engine implementation and test that separately as shown below:

![](attachments/Pasted%20image%2020240611093737.png)

For interest:  Did you know that IoC containers are changing a lot recently. In the past, they’ve largely been built using a technique known as ‘reflection’, but they are now being extended so you can choose between reflection-based dependency injection, which is far more dynamic and flexible for Just-in-Time (JIT) and dynamic binding, and static injection, which plays far nicer for scenarios that require Ahead of Time (AOT) compilation.  This is a big topic, but as ever, there are trade-offs between the different approaches which ultimately depend on your use-case.

### Avoid Inheritance Tax - Prefer Object Composition, Interfaces, Traits, Mixins to achieve Ad-hoc Polymorphism

- Deeply nested inheritance hierarchies with sub-classes that extend super classes have their place, for example when developing libraries and frameworks. For application developers however, they are often considered a bit of an anti-pattern these days. This is because inheritance strongly couples you to the structure of the classes in the parent hierarchy, which sometimes forces you to implement abstract methods that you don’t actually need. In these scenarios, you may need to throw some form of unsupported exception or error.  IF you have access to the source code, you can extract those parent methods into a new parent in the inheritance hierarchy and inherit from the appropriate level – i.e., from the direct parent if you do want those methods, or from a higher-level ancestor if you don’t need the newly extracted methods (effectively skipping the newly extracted parent layer). However, this means all existing sub-classes may need to be refactored to inherit from the appropriate parent or ancestor class – this can be a very expensive refactor and can get complex with deeply nested hierarchies. In short: Inheritance asks you to bundle all common elements into a parent class - but this means that exceptions to the common can require expensive refactoring.

- To discourage inheritance, in modern languages, classes are ‘closed to extension by default’ - often you have to explicitly allow extension with special keywords such as the ‘open’ class prefix in Kotlin. This makes SOLID’s ‘Open for Extension, Closed for Modification’ best-practice explicit in the language.  Rather than pay the inheritance tax, use more flexible approaches such as Composition, Interfaces, Traits & Mixins.

- **Composition / Delegation**: A complex term that in practice simply means that one class contains or is passed an instance of another to use its capabilities. Note that composition alone does not establish a polymorphic type without the addition of an interface or trait.

- **Interfaces / Traits**: A single class can implement multiple interfaces or traits to achieve ‘ad-hoc polymorphism’ and sub-types. This is a far more flexible approach for a type to be a subtype of something else. Depending on the specifics of the language, the interface/trait may provide abstract function declarations, functions with default implementations, and optional member variables. Note that for default implementations, the interface/trait can only depend on elements defined within the interface/trait itself.    

- Side Note: Object composition with optional ad-hoc polymorphism might be the only option available to you if you don’t own the source-code of the object you want to extend as the original object might be closed for extension. Therefore, the only way to extend such an object is to compose it within another object and hide its functionality behind optional interface(s).

- Dynamic languages like Python have ‘Duck Typing’, this is similar to using interfaces but without having to define an explicit type contract using keywords such as ‘class Ducky implements interface Duck’. Instead, if the code has the relevant function with the required function signature, it will be invoked, otherwise a runtime error is generated (“if it walks and swims like a duck, it’s probably a duck”).  This is also known as ‘Structural Polymorphism’. The ease and speed of development that duck typing brings is great for scripting and smaller code bases, but as the size and complexity of the code increases, you may want to consider moving to a more strongly typed language that brings more compile-time checking (note, you can always introduce static typing such as Python’s optional type hints).

- Certain languages don’t even support object extension, relying instead on ad-hoc polymorphism (e.g., Rust/Go).

- Languages may have additional ways to extend the functionality of an existing type. For example, C# and Kotlin support extension functions.

- **Sealed-Interfaces**:  Some languages take interfaces a step further by allowing the developer to define ‘sealed interfaces’ which specifically list _which_ classes which are allowed to implement that sealed interface. Sealing is great for defining ‘exhaustive’ when & case patterns for Algebraic Data Types (see ADTs below).

Each of these patterns are useful in different circumstances depending on whether your aim is to define polymorphic types, adding new functionality to existing objects, or sharing / re-use of method and state.

### It Should Not be Possible to Create an Object in an Invalid State

Nuff said.

### Know Some Design Patterns

There might be a tried & tested design pattern for the problem you’re tackling. Some patterns are probably overkill, but some genuinely useful patterns include Factory, DTO, Observer, Strategy, Singleton, Repository, Stateless Façade, Visitor. Have a look at the recommended texts in the appendix. GoF is kinda old school these days.


![](attachments/Pasted%20image%2020240611094028.png)


#### The Strategy Pattern Example

This pattern abstracts logic behind a common abstraction such as a SAM interface (Single Abstract Method interface) so that an implementation can be **chosen at runtime**. This makes the code more flexible and reusable. In the Kotlin example below taken from [Dave Leeds](https://www.youtube.com/watch?v=-Ak44LFwlwI&t=64s), we use validation as an example, where any of the validators can be passed at runtime to the FormField class. 
![[Pasted image 20240905161915.png]]
Here are two more Kotlin examples that are more idiomatic which reduce boilerplate, again from Dave Leeds: 
![[Pasted image 20240905162859.png]]
An even more concise example:
![[Pasted image 20240905163103.png]]
Note you can use an extension function to easily create an optional version:
![[Pasted image 20240906135416.png]]

At the call site: 
![[Pasted image 20240906135400.png]]

#### The Visitor Pattern

The visitor pattern is used to separate business logic from objects on which they operate. Typically, objects define an accept method then call method(s) on the accepted visitor. The calling object is typically passed to the visitor as an argument so the visitor can access the object's public state, as in the pseudo code: `accept(Visitor v) { v.visitDoLogic(this); }`.   New logic can easily be added to the visitor's `visitDoLogic(callerObj)` without having to update the calling objects which illustrates an example of the open closed principle in SOLID.   This pattern uses a double-dispatch logic: first an object's `accept(Visitor)` method is invoked, then the visitor's `visitDoLogic(obj)` method second. 

The visitor pattern is typically invoked for large cascading / nested object trees; an `accept` method can pass the visitor instance to all its member objects that also define an accept method, for example: 

```c#
public class Addition : Expression {
  public Addidtion(Expression left, Expression right){
    Left = left;
    Right = right;
  }
  public override void Accept(Visitor v) {
    Left.accept(v);
    Right.accept(v);
    v.vist(this);
  }
 // get values etc elided 
}

// invoking code would create a Visitor implementation and invoke the double dispatch logic by calling `Addition.Accept(visitor);`
```

Languages implement the visitor differently. For strongly typed polymorphic languages that support method overloading (Java, C#, Kotlin), interfaces can be used simplify the double dispatch logic as `accept` and `visit` methods can be overloaded using different argument types.  Languages that do not support polymorphic overrides e.g., Go and Python, typically need to define different visit-method names e.g.

```Go
type Visitor interface {
	visitWheel(wheel Wheel) string
	visitEngine(engine Engine) string
	visitBody(body Body) string
	visitCar(car Car) string
}
```

### Consider using the Builder Pattern for More Complex Object Creation Scenarios

Builders are especially useful if the dependencies of your class have complex invariants. Basically, this means that if you class can only be constructed with a particularly complex combination of dependencies such as ‘my object requires A and B and either C, D, or E and F, but never G if D is present’ (I’m sure you get the idea), then the Builder pattern can help you.

### Information Hiding

Your first instinct should be to make a method/member/variable private first, then increase visibility as required, not the other way around.

### DRY – Do not Repeat Yourself

Duplicating chunks of code is odorous - don’t do it.

### YAGNI – You Are not Going to Need It

Following Agile processes (i.e., ‘Feedback Driven Development’) should trap and prevent unnecessary code.

### Comment in-line As You Go

You don’t retrospectively comment your code, you just don’t. Using sensible names should prevent long-winded doc strings.  Use xDoc tools e.g., PyDoc, JavaDoc, xDoc etc. Document the _intent_ of the function/class, not the implementation details.

### The Boy Scout Rule

Leave code in a better state than you found it & don’t comment bad code, re-write it with good descriptive names.

### Law of Demiter and Train Wrecks

A module should not know about the innards of the objects it manipulates (by ‘objects’, I don’t mean immutable DTOs / records / data objects, structs, I mean genuine objects that have behaviour that works on their state). [https://en.wikipedia.org/wiki/Law_of_Demeter](https://en.wikipedia.org/wiki/Law_of_Demeter)

![](attachments/Pasted%20image%2020240611094151.png)
From Uncle Bob’s ‘Clean Code’

### Functional vs OOP - Choose Two

Despite recent trends where the functional paradigm is gaining popularity, adopting a pure functional language might be a stretch – IO, mutation and OOP are ubiquitous and are here to stay. For example, many sorting algorithms are simply faster and more memory efficient if you (safely) mutate ‘in-place’. However, if the language permits and supports functional concepts, taking a more functional approach is highly recommended. Within the same code base, the general recommendation is to keep the OPP and FN code-paths cleanly separated, aim for a core of pure functions, and push out ‘side effects’ to the out boundaries of your code (see the Dependency Rule / Bullseye) so they become intended ‘effects’ and not nasty interleaved ‘side effects’.

Well worth a watch: [https://www.youtube.com/watch?v=HSk5fdKbd3o&t=543s](https://www.youtube.com/watch?v=HSk5fdKbd3o&t=543s)

### Do not pollute Functional Code with Mutable State

importantly, keep functional call chains pure; you really don’t want to pollute your functional code with shared mutable state across threads. Consider the following example - one is broken, the other is ok, the difference is subtle.  So, while combing FN + OOP is powerful, be very careful.

![](attachments/Pasted%20image%2020240611094242.png)

ParallelStream will split work across a thread pool, this means the list, which is not atomic and is our ‘external mutable state’, is subject to a whole host of complex threading issues (overwrites, ghost-reads, race conditions etc). Replacing the forEach with a functional ‘reducer’ operation (e.g., toList()) and converting the call chain from a statement to an expression solves the issue - there are no statements in _pure_ functional code.

### Make Immutability your Default

- Global Mutable State is evil and will eventually cause you problems.
- Modern languages are immutable by default, for example, in Rust and Kotlin, you need to specifically ‘opt into’ mutable variables using special keywords such as ‘mut’, and use of ‘var’ (immutable) instead of ‘val’ (mutable).

### Interior Mutability

When mutability is necessary, try to encapsulate it within a function so any mutable state is not leaked. This is known as ‘Interior Mutability’.  To do this, you’ll likely need to take defensive copies of the input parameters to reduce the risk of side effects.

### Use Calculations Where Possible to Limit Side Effects

Calculations or ‘pure functions’ have no side effects and are ‘idempotent’. This means given the same input arguments, idempotent functions will return the same value regardless of _when_ you call them (incidentally, this means you can cache the results of expensive calls in a lookup table, for example).  Operations may have side effects such as updating a database, writing a file to disk, calling a remote service, even printing to the console is a side effect (maybe another process is reading your stdout/err?). Operations return values may change depending on _when_ you call them. For example, consider calling a travel service with the same function parameters – results will probably vary depending on the time of year or time of day. One approach to help make code that depends on randomness (e.g., a random number is required), is to provide a seed value, and make that part of the external API which allows you to test using the same seed values.

### Separate Operations from Calculations

See prior bullet. Functions should either do something such as create side effects (operations) or provide an answer to something such as returning a result from a stateless calculation. Try to separate functions that have side-effects from pure functions (aka calculations) using a naming convention. Try not to do both in a single function.  In some languages you can be explicit– e.g., in Kotlin, a common convention is to reserve single expression functions only for calculations – you can quickly/easily see this in the first line of the expression function signature – no need to understand the function body for side effects. Nevertheless, unless you are using a pure functional language, this is still only a convention.

### Error Handling - 4 Types of Problems

1.     Unrecoverable problems: Is the error recoverable? If not, then let the program crash. For example, a FileNotFound exception/error/panic should crash if that file is the application's mandatory config file - you can't continue without it.

2.     Recoverable problems: For example, if a remote service is temporarily unavailable, you could introduce a retry before showing an error to the user.

3.     Errors that need to propagate to the user:  An error-as-value would be suitable if you are building a file-explorer GUI - you don’t want your program to crash if a file gets deleted by another process. In this scenario use a value-error or catch the exception and convey a sensible message to the user.

4.     Programming mistakes:  Let the program crash, you’ll be motivated to fix the problem quickly. These are typically runtime errors/exceptions/panics.

### Error Handling - Exceptions vs Errors-as-Values

Errors as values vs exceptions is a hotly debated topic in programming communities:

***Proponents of errors-as-values:*** 
- Fans of errors-as-values argue that function return types that wrap either a success OR failure value is the more reliable approach to error handling because you are explicitly forced to handle errors immediately, typically using a conditionals to test for error or success. This ensures error handling is not an afterthought. 
- Supporters also argue that there is less uncertainty compared to throwing exceptions because it can be challenging to determine all the exception types that can be thrown by a deep call stack. Also recognise because unhandled unchecked exceptions do not create compilation errors, the compiler can't help you discover all of the different types of unchecked exception that could be thrown, unless you dig and read all the docs that is.
- Another issue of a specific type of exception known as a 'checked' exception is that they prevent functional composition. This is because the compiler forces you to handle checked exceptions wherever they can be thrown, but they are not considered as part of a function's return signature and type system. Instead, exceptions invoke orthogonal flows that 'break out' of your regular functional flow. Checked exceptions therefore breaks 'referential transparency' (see discussion below on Error Monads such as `Either` & `Validated`). Checked exceptions are generally not recommended these days, except for certain special use-cases where they still have their supporters.

***Proponents of exceptions:*** 
- Fans of exceptions argue that by forcing you to interleave error checking at function call sites throughout your code obscures the code's happy path and readability.  
- Exception fans also argue that exceptions centralise your error handling code which gives a clean separation of concerns. 
- For low-level code, exceptions are largely considered an effective strategy for surfacing underlying issues such as low level operating system issues which may be mistakenly obscured by the errors-as-values pattern (although the same could be said by mindlessly catching all exceptions). 
- When used correctly and with discipline, exceptions can also be more performant than pervasive and interleaved error-value checking. This is because languages like C++ and Java have 'zero cost exception handling.' I think this is a misleading term, what it actually means is zero cost to the happy path code provided no exceptions are thrown. Assuming no exceptions are thrown, quite simply, there is less for your code to do as there are no interleaved conditional error checks. While any performance hit from interleaved result checking is likely to be marginal for the majority of use-cases, it may become more pronounced in deeply nested code or tight compute loops. However, this can be mitigated with good code structuring by moving error checks out of and before any performance critical-sections.   

- Whether to use exceptions has profound implications on your API design and performance, be aware of the issues highlighted above. Some modern languages, e.g. Mojo, go as far as trying to address any choice for you by compiling exception handling code under-the-hood to use errors-as-values. I think the aim is to allow you cleanly separate the happy path from exception handling code (clean separate of concerns) while allowing you to retain the performance of error-as-values should an exception be thrown. At the time of writing, it is too early to tell if this is a successful strategy, but if anyone can do it, Chris Lattner can.

- Of course, choice between exceptions or errors-as-values depends on the language and environment - you don't get exceptions support on every architecture and platform. The result pattern is much more flexible especially on embedded systems. 

***Can I use both:*** 
- Yes, depending on your language of choice and what is considered idiomatic. Some modern languages support both approaches. For example, to support interoperability with Java, the Kotlin language supports unchecked exceptions as well as its own `Result` type which is intended for low-level code rather than for modelling business errors. For modelling business errors, they recommend using sealed class hierarchies that introduce exhaustive pattern matching to handle errors (see discussion on data oriented programming). 

- At the time of writing, a dedicated union type for capturing a result OR one or more errors is on the Kotlin roadmap. 

***Hybrid Approach:*** 
- Languages may also support more advanced error handling strategies. For example, the Kotlin Arrow2 library simplifies the use of OOP and Functional error handling within the same code base (Functional vs OOP? - choose both). For example, lower level code can apply `try/catch/finally` blocks for localised exception handling and recovery if needed, while higher level calling code can provide a wrapping `error context` that can be used at the boundary; Rather than throwing exceptions at the boundary (between different layers of code), exceptions can be *raised* into the higher level error context. Raising rather than (re)throwing then allows the raising functions to be composed within functional compositional call chains - raising does not break referential transparency. In the top layer of your code, such as in a top-level service facade or global error handler in a webapp, you would then need to handle the exceptions raised within the error context, such as performing a transaction roll back or performing a retry.  For a great presentation with examples, see this great talk from Simon Vergauwen from [Kotlin Conf 2023](https://youtu.be/JcFEI8_af3g?si=vH5OG86JTQWFrGnw) 


### Error Handling - Exceptions should not be used for flow control, use errors-as-values instead, exceptional != conditional

Passing around a deeply nested stack trace within conditional and control logic is very expensive, don't do it. Instead, model your (known) business errors as values (no need to pass around exceptions), and leave exceptions for coding errors and exceptional situations. If you want control flow logic that says "if success do this..., but if an error occurs then do this..." then use the result pattern.

### Error Handling - Only use exceptions for exceptional situations such as coding errors and unexpected situations, exceptional != conditional

For example, an invalid object posted to your API is not exceptional, this should be handled as a potential business error. In the situation where some code throws an exception such as a parse error, catch it locally, extract the useful information, and return an error-value. In general, the result-as-value pattern is appropriate where the problem is the fault of the caller and not a programming mistake e.g., invalid input / form data.

### Error Handling - Provide Relevant Exceptions for the abstraction layer

If you use exceptions (not all languages have exceptions e.g., Rust, Go), define Exceptions in terms of a caller’s needs and wrap 3rd party library APIs including their exceptions. Often, only a few custom exception classes are needed for a particular area of code.

### Error Handling - Bubble Exceptions Upwards or Trap at Source

Generally, pushing exception handling code up to the ‘outer layers’ of your code toward the boundaries is usually a good approach. It also helps cleanly separate the ‘happy path’ from interleaving error handling code.  However, this isn’t a hard rule, in some situations you may need to try/catch/finally at the source of the error to take important corrective actions such as closing an IO resource or rolling-back a DB transaction.

### Error Handling – Model the Absence of Value Explicitly

This largely depends on the language you are using:
- Nullable languages (C/C++/Java): Dereferencing a null pointer causes bad things to happen. This is known as ‘the billion-dollar mistake’ coined by Tony Hoare, in 1965.  In your code, be sure to make it clear when null is meant to represent the ‘absence of value’ (e.g., with @Nullable annotations for example). If you must, when using a 3rd party library for example, be ‘defensive’ such as checking for nulls using if statements where appropriate.

- Side Note: polluting your code with defensive checks is often considered dirty, but sometimes you just have to it if no other approach is available.

- Side Note: many argue null is an acceptable way to represent the absence of value, it’s just a fact and is too fundamental in many layers.  They argue the real billion-dollar mistake is not null itself, but in the failure of the language to do type-safe handling of null.

- Some languages e.g., Kotlin, Rust & Python have ‘safe nullability’ baked into their type-systems e.g., None to mean no value and optional ‘?’ on variables (‘var?’) to indicate this variable might be null.

- Functional languages commonly use ‘Either’ monads to wrap errors and the absence of value. With this pattern, the programmer is forced to handle the occurrence of no value, it can’t be ignored, mistakenly or otherwise, as with defensive programming. See below.

- Various modern languages use Algebraic Data Types (an extension of the ‘Special Case’ pattern) and return types that wrap either a successful result or an error/null (Go, Rust, Kotlin, modern Java):

·       [https://martinfowler.com/eaaCatalog/specialCase.html](https://martinfowler.com/eaaCatalog/specialCase.html)

- Algebraic Data Types (ADTs) are generally either ‘Sum Types’ or ‘Product Types’ and are excellent for representing multiple special cases, including multiple error states.

### Error Handling in Functional Programming – Error Monads such as Either & Validated

In functional languages monads are widely used to chain a sequence of function calls into a clean ‘happy path’. This is also known as ‘functional composition’ or 'effect orientated' programming. A core tenant of functional approaches is to produce more declarative and expressive code over classical imperative approaches which usually interleave error handling with the happy path. In functional approaches, you define ‘what to do’ with functions, not ‘how to do it’ as with imperative approaches.

An `Either` monad wraps either a result type or an error type, but not both, typically (`Either<LeftError, RightSuccess>`). Note that Rust is opposite, where left is success and right is error. An instance of a monad is passed between functions in a call chain. Wrapping errors within the ‘monadic context’ allows the functional call chain to be composed without polluting and breaking the chain with exceptions and error handling code.  If a function returns a LeftError wrapped in the Either, subsequent functions in the chain will short-circuit and will simply return the erroneous Either. This continues until the end of the call chain is reached. 

A `Validated` monad aggregates errors or exceptions within a functional call chain. The purpose is to capture all the errors rather than short-circuiting on the first. A simple example would be capturing all the errors on a form, rather than returning early on the first erroneous form entry. 

Regarding exceptions in functional composition: If your language uses ‘Checked Exceptions’ (e.g., Java or when using other JVM languages that call out to underlying Java libs), you can’t throw checked exceptions during functional composition as they force you to handle the error and break the call chain with try/catch or throws statements. In this scenario, wrap the exception in the monadic context and return a LeftError. Note that throwing _unchecked_ exceptions is ok in a functional call chain as they don’t pollute the happy path with try/catch or throws, but you likely still want to wrap the error in the monadic context to return an error-as-value, e.g. if that error is not a programming error or is an exceptional circumstance.

### Data Orientated Programming with Algebraic Data Types (ADTs)

The Special Case Pattern is one example for modelling your domain types in such a way that the absence of value is explicitly modelled in your domain making illegal program states and crashes more unlikely. However, you can go a lot further using Algebraic Data Types (ADTs). ADTs include both ‘Product Types’ for modelling aggregation such as a C’s or Golang's ‘struct’, and ‘Sum Types’ for modelling choice, also known as ‘Union Types’ or ‘Tagged Unions’.

This simple combination of aggregation and choice is deceptively powerful and shows up in many programming languages to model domains, return types and function arguments:

- Product Types are great for modelling aggregation, and include immutable data classes such as records, data objects, structs and traits. They are called ‘Product Types’ because their state ‘when considered as a whole’ is the cartesian product of their data. 

- Sum Types can be used to represent choice and are polymorphic but with a fixed set of implementing subtypes (e.g., ‘sealed’ classes or interfaces in Kotlin/Java, ‘enum’ in Rust, ‘Union’ types in Python). They are called Sum Types because the set of possible types is the sum (union) of the total allowable set.

In modern languages, ADTs combine with ‘de-structuring’ and ‘pattern matching’ to facilitate powerful ‘exhaustive’ pattern matching constructs with when & switch statements. Exhaustive matching means the compiler will generate a compilation error if not all sum types are explicitly handled.

### Concurrency & Parallelism

A big topic, it can’t all be covered here, it ranges from single host shared memory parallelism, to multi-node clustering (e.g., HPC), to distributed multi-host cluster networks (e.g., remote Actors / FaaS). Concurrency is not parallelism. Concurrency is a software concern – context switching on a ‘carrier thread’ using continuations or virtual-threads gives the illusion that multiple things are happening at once, while true parallelism is both a software and hardware concern – you need hardware to support parallelism which can range from multiple cores on one CPU, multiple CPUs, nodes, local and remote actors, remote VMs, cloud functions such as Lambda and more. Here are some general recommendations:

- Keep platform threads as isolated as possible & limit mutable global state
	- Sharing of fixed immutable state is fine.
	- Taking defensive copies of data can help prevent race conditions and other ‘spooky actions at a distance’.
	- Try to be more functional and limit your use of global mutable state.
	- Understand the pitfalls of multi-threaded code such as race-conditions, ghost reads, dirty reads, dirty writes, and deadlock.  

- Keep synchronized critical sections as small as possible (Amdahl’s law)
	- Even a small amount of synchronization *_significantly*_ affects performance. See Ahmdal’s law. For example, even if your code is 95% parallel, only 5% synchronisation means throwing more processors at the problem doesn’t improve speedup beyond ~256 processors.
	- Understand the pitfalls of multi-threaded programming. If deadlock, live lock, ghost-reads, dirty reads, and atomic vs composite actions don’t make much sense to you (do you think ‘i++’ is atomic? – no it isn’t), then you will no doubt run in to problems. These days, there is often a much better approach to coding low-level multi-threaded and shared memory models.
	- When testing, use more threads than processors – running with more threads than processor cores to encourage task swapping. The more frequently your tasks swap, the more likely you will find issues.

![](attachments/Pasted%20image%2020240611095042.png)

#### Know the difference between IO bound tasks and CPU bound tasks and their common solution patterns

- IO Bound Tasks require asynchronous patterns to achieve concurrency. The solution patterns include:

	- Async/Await and Coroutines (e.g., Go’s ‘Goroutines’ and Kotlin’s suspending functions). These are often referred to as ‘coloured approaches’ because your code is split into two; red functions reflect asynchronous code paths and blue functions for synchronous code paths – here’s the original and now famous blog: [https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function)
	- Virtual-Threads / Fibres such as the ‘colourless’ Project Loom for the JDK where the virtual threads are implemented in user space using continuations: [https://wiki.openjdk.org/display/loom/Main](https://wiki.openjdk.org/display/loom/Main)
	- Continuations are very low-level and are used to implement patterns such as coroutines & virtual threads. Continuations can be suspended, stored on the heap, and restarted. Typically, you wouldn’t code directly with continuation APIs (CPS – Continuation Passing Style), although some languages do have public APIs for them.
	- Async-Wrapper types such as Futures & Promises. Note that these are really just higher-level synchronisation primitives, and the task that you await itself would need to be non-blocking to achieve high levels of concurrency.  
	- Call-Back functions (beware ‘call-back hell’ as often seen in JavaScript). In fact, more friendly ‘syncrhonous’ patterns such as Coroutines and Continuations simply abstract much of the lower-level call backs from the programmer.
	- Use non-blocking IO libs & APIs instead of blocking libs.
	- Reactive Frameworks e.g., project Reactor.

- For CPU Bound Tasks, the solution patterns include:

	- Platform/OS/POSIX/Kernel threads (pthreads). These are good for numerical computations that don’t require (much) IO, such as ‘tight computational loops’. Note that platform threads are heavyweight and should be pooled for effective resource usage. For example, on Linux, each thread typically requires ~1MB of mem per thread – this is because they are controlled by the OS, and the OS has to be generic enough to handle a variety of use-cases, so 1MB was assumed to be a good default. Platform threads they are _very_ different from Virtual Threads in terms of how they are implemented.  A platform thread is very similar to a process in terms of resource-cost, except that threads allows memory sharing between multiple threads while multiple processes do not share the same memory space. For multiple processes, you need some other mechanism to share state and messages between processes, such as a common memory-mapped file(s), filesystem, databases, and message brokers.
	- Shared memory frameworks such as OpenMP which make multi-threaded programming simpler by avoiding low-level synchronisation primitives.  
	- Fork-Join-Pools and related ‘Work Stealing’ patterns that involve task queues. FJP is a highly recommended way to achieve best performance because low-level Kernel/OS threads can be re-used.
	- Horizontal scaling with Pub-Sub and Competing Consumers. This is where multiple compute nodes subscribe to a message channel and pull messages from the channel. If the queue-depth gets too high, you add more consumers to process the messages.
	- Lazy Parallel Streams in functional approaches.
	- Message Passing e.g., the Actor model (e.g., Akka) & Message Passing Interface (e.g., OpenMPI) in HPC are both examples of message passing. Note that the Actor model is actually the canonical parallelism pattern, while MPI is quite niche (largely just the HPC community).
	- If you must use low-level locks and synchronization primitives with critical sections, try to use ‘re-entrant’ locks for better composability and performance over non-re-entrant synchronized blocks. Check if the languages mutexes (semaphores, count-down latches) are re-entrant.
- If you must use low-level locks and synchronization primitives with critical sections, try to use ‘re-entrant’ locks for better composability and performance over non-re-entrant synchronized blocks. Check if the languages mutexes (semaphores, count-down latches) are re-entrant (language agnostic advice).

### Security Development Practices

- For Hartree folks, if using a cloud hosted development environment, you must consult and agree to the practices given in [Hartree’s Cloud Acceptable Use Policy](https://stfc365.sharepoint.com/:w:/r/sites/HartreeIGaA/ISO27001/Information%20Security%20Management%20System/Policies/HCIS-0044-plc-01.1-Cloud%20Access.docx?d=wc3116a720ba941cc9de27c0029df1932&csf=1&web=1&e=zkvG1Q) document. All polices and related information can be found [here](https://stfc365.sharepoint.com/sites/HartreeIGaA/ISO27001/Forms/AllItems.aspx?csf=1&web=1&e=ZWjlia&siteid=%7B938CBF09%2D9359%2D4BB6%2DB56D%2D55D938C510C1%7D&webid=%7B02B09618%2D9B45%2D479E%2DBFA4%2D9DEE7B833CA6%7D&uniqueid=%7B8BE63CF1%2DD73D%2D441D%2D9570%2DCD95D488D38A%7D&RootFolder=%2Fsites%2FHartreeIGaA%2FISO27001%2FInformation%20Security%20Management%20System&FolderCTID=0x012000AFB03BC45914F7439215AF3907065BF0).![](attachments/Pasted%20image%2020240611100053.png)

- Never add plain-text credentials including username/passwords and ssh keys/tokens into version control.
- Sensitive data such as credentials can be stored locally in your local dev environment using ephemeral sources such as environment variables, command line arguments, local files such as '.env' files that are git ignored (use a '.gitignore' file) to ensure they are not committed to VCS, and local key-chains / credential stores such as HashiCorp's vault and [https://github.com/openbao/openbao](https://github.com/openbao/openbao) .
- Do not hard-code secrets in code. For production, use well-established secret serving methods such as creating a secret object in OpenShift that configures environment variables for your running pods.  
- Public URLs should always be secured using TLS/HTTPS. Host certificates can be freely obtained from [https://letsencrypt.org](https://letsencrypt.org)  
- Always consider linting and scanning your code for vulnerabilities and anti-patterns using well-established tooling such as FindBugs, Snyk for containers, OWASP's Dependency Check tool suite: [https://owasp.org/www-project-dependency-check](https://owasp.org/www-project-dependency-check)  
- Familiarise yourself with OWASP's Top Ten security risks for webapps: [https://owasp.org/www-project-top-ten](https://owasp.org/www-project-top-ten)  
- Always update default passwords that are shipped with products e.g., 'admin' is sometimes used default username and password pair.
- To minimize injection attack surface, don’t use your own variable binding or hardcode parameters using string concatenation – use the supported variable binding tooling to ensure values are always escaped.



## Agile Process Guide a.k.a Feedback Driven Development

For the Hartree Centre, we propose an Agile methodology as it largely suits the type of projects we do. Agile is an overused term, so for Hartree’s purposes, a good definition is ‘Feedback Driven Development’.  Iteration and customer feedback really ARE essential if we are to successfully address real customer needs. Know that industry data shows that even for the best software companies in the world, two thirds of their ideas produce zero or negative value so continuous feedback is essential to mitigate the risks: Online Controlled Experiments at Large Scale: [http://ai.stanford.edu/~ronnyk/2013%20controlledExperimentsAtScale.pdf](http://ai.stanford.edu/~ronnyk/2013%20controlledExperimentsAtScale.pdf)

According to the values of the original Agile manifesto (search the original ‘Snowbird meeting’), agile development practices include risk-taking, rapid-feedback, frequent and high-bandwidth communications across the whole team, and collective project ownership. This means full stakeholder involvement with everyone: developers, testers, scientists, end-users, and business-development managers. It emphasises ‘individuals and interactions over processes and tools.’ Agile is especially relevant for greenfield and relatively short-lived projects which describe many of the projects we do at the Centre.

We recommend weekly or fortnightly iterations involving customer playbacks and demos. Anything longer than 2 weeks can require significant course correct if/when you go in the wrong direction - agile aims to catch problems early and to course correct.  According to Uncle Bob Martin, the emergence of agile was to “find out how screwed we were as early as possible, it wasn’t just about writing software quickly”.

### Design Thinking Workshops and Scoping Document

Design Thinking puts you in the shoes of the customer so that you can understand their pain points. This helps design solutions that really address customer needs. Hartree have a set of recipes for activities that you can use to conduct [DT workshops](https://stfc365.sharepoint.com/:f:/r/sites/TheHartreeCentreCommunityChannel/Shared%20Documents/Hartree%20Communications/Business%20Development/Design%20Thinking%20Workshops?csf=1&web=1&e=cpdeqS). The activities don’t have to be applied religiously and you can adapt as needed. The activities include As Is Scenario Journey Map, Empathy Maps, User Persona and Problem Statements, User Stories, Ideation, Prioritisation, Ideal To-Be Scenario Journey Map, Outcome Statements, Cupcake Roadmaps.

Hartree also has a [scoping doc](https://stfc365.sharepoint.com/:f:/r/sites/TheHartreeCentreCommunityChannel/Shared%20Documents/Hartree%20Communications/Business%20Development/Design%20Thinking%20Workshops?csf=1&web=1&e=cpdeqS) that you can send the customer ahead of time to help focus minds.

### Epics and Work Package Span Multiple Sprints

Epics are like Work Packages. Typically, they require multiple tasks and span multiple sprints.

### Define user stories either with the INVEST Framework, Who-What-Why or the Connextra Card Template – all are good and you don’t need to be too rigid

-  _“As userType [X], I need a way to do [what?] so that I can [what’s the benefit]”._

- _Who, What Why_

- INVEST:
	- Independent - this means we try to design stories that do not need to be implemented in a particular order (a soft rule as there may well be stories that need to be prioritised).
	- Negotiable - to retain agility, we recognise that requirements often/inevitably evolve and so we don't focus overtly on getting the details right up-front (i.e., Waterfall).
	- Valuable - must have a clear and quantifiable benefit to the client.
	- Estimable - a story must be concrete enough that developers can estimate it.
	- Small - a story must not be larger than one or two developers can implement in a single iteration.
	- Testable - when a developer says that its 90% ready, nobody really knows how close it is to being finished.
  
### Arrange core user stories into a Journey Map with a narrative flow or backbone of Big Activities moving from left to right

Beneath each big activity, define short verb phrases to describe what the user does to achieve each big activity.

![](attachments/Pasted%20image%2020240611100610.png)

### Task Backlog

Create a list of tasks and use ‘planning poker’ / finger-waving to estimate effort – after a ‘3, 2, 1’ countdown, everyone at the same time provides an estimate of the difficulty of a task between 1 and 5 or holds up a card. This ensures honest estimates from everybody which is Important because different team members may have different experiences/specialities of the task area. See [https://www.evernote.com/l/AWQ6FGRtfrNI1az21FVp9aosQ9zu8b-4CXg](https://www.evernote.com/l/AWQ6FGRtfrNI1az21FVp9aosQ9zu8b-4CXg)

### Requirements Document and System Architecture Document

### 1-to-2-week Sprints  

Break up the Backlog into sprints to deliver your cup-cake roadmap. Provide effort estimations for tasks using ‘planning poker’. More than two weeks generally gives enough time for software to deviate without requiring significant refactoring and course-correction, so we don’t recommend more than 2 weeks.

### Inline Testing

Test the critical path and be pragmatic about coverage - 80% coverage often not feasible or even useful. Develop tests in-line with the mainline branch. TDD helps us think about the public interfaces / API to the code under development.

### Demo and Playbacks

At the end of the sprint, demo your progress to the client. This is important. Agile can be paraphrased as ‘Feedback Driven Development’.  It is essential to get that customer feedback early and continuously.

### Acceptance, Sign Off and Cucumbers 

- If possible, get the client to sign-off work every month (PMO have a ‘Decision Point Review’ template).
- Use the Cucumber approach for acceptance testing i.e., ‘Given, When, Then’.  For example: ‘Given [a particular context/scenario], When [something happens], Then [this is the result]’.

### Iteration and Incrementalism

- Recognise that we need both iterative & incremental approaches to building complex systems. Incrementalism == modularity, which helps break down complexity.
- Review the Backlog, revise and plan your next sprint, jump to 7.

![](attachments/Pasted%20image%2020240611100815.png)

### Cup Cake Roadmaps

- Plan a cup-cake dev roadmap. A cup-cake won’t feed everyone, but it can have core ingredients - it’s a whole-product that a user can taste sooner rather than later.
- If the cup-cake tastes good, proceed with the vanilla sponge, hopefully ending with the multi-tier wedding cake that can feed everyone. Iterate your development roadmap and keep soliciting user feedback.

Thanks for reading, comments/feedback most welcome. Have fun !

## Appendix - Recommended Texts 

![](attachments/Pasted%20image%2020240611100930.png)

![](attachments/Pasted%20image%2020240611100944.png)


---
