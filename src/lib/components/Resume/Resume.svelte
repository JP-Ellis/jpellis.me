<!--
Resume component, used to display a resume in a nice way and makes use of the
Section component to display each section of the resume.
-->
<script lang="ts">
  import KPMGLogo from "$lib/assets/images/company/kpmg.svg?component";
  import MelbourneUniversityLogo from "$lib/assets/images/company/melbourne_university.svg?component";
  import Organization from "$lib/components/Resume/Organization.svelte";

  import Position from "./Position.svelte";
</script>

<!-- Resume -->
<section id="resume" class="flex flex-col justify-center">
  <h1
    class="p-6 mb-12 text-3xl md:text-5xl font-bold text-center shadow-xl bg-surface-100-800-token"
  >
    Resume
  </h1>

  <Organization title="KPMG" location="Melbourne, Australia">
    <KPMGLogo slot="logo" />

    <Position
      title="Senior Consultant"
      subtitle="Data & DevOps Engineer"
      start="{new Date('2022-06-27')}"
      end="{null}"
      keywords="{[
        'azure',
        'infrastructure-as-code',
        'devops',
        'data engineering',
      ]}"
    >
      <p class="unstyled text-lg md:text-xl">
        I have successfully delivered multiple complex projects in various
        industries. These projects involved:
      </p>
      <ul class="list-outside list-disc pl-8 text-lg md:text-xl">
        <li class="my-4">
          Deploying <strong>machine learning infrastructure</strong> within a
          tightly regulated environment, processing
          <strong>highly confidential</strong> information, while working closely
          with stakeholders across cloud infrastructure, security, and the data science
          team to ensure a good outcome. This project has also been recognized for
          pioneering the deployment of machine learning infrastructure within KPMG
          globally.
        </li>
        <li class="my-4">
          Implementing a <strong>data analytics platform</strong> from scratch,
          using Azure and Snowflake, and integrating
          <strong>CI/CD workflows</strong> in Azure DevOps to automate deployment
          of documentation and Bicep infrastructure-as-code. This project included
          implementing two model data pipelines as starting points for the client,
          taking Excel and API data through to a curated layer.
        </li>
        <li class="my-4">
          Conducting a thorough <strong>review</strong> of a university's
          Integration Capabilities Uplift program’s platform, with a focus on
          the business need, clarity and scope driving the implementation, the
          capabilities being provided by the platform, and the suitability of
          the technology design. Our team provided the client with
          <strong>insightful recommendations</strong> as to the program's strengths
          and areas of improvement.
        </li>
        <li class="my-5">
          Acting as the <strong>Technology Lead</strong> and
          <strong>Scrum Master</strong> for a foundational and modular Azure Data
          Platform accelerator and a configuration-driven orchestrator framework
          for Data Factory, building upon the former accelerator. I also developed
          a modular and extensible Python framework to facilitate the deployment,
          configuration, and development of the Azure Data Platform accelerator.
        </li>
      </ul>
      <p class="unstyled text-lg md:text-xl">
        Overall, I have a strong track record of <strong
          >delivering innovative and effective technology solutions</strong
        > while collaborating closely with stakeholders to ensure successful outcomes.
      </p>
    </Position>
  </Organization>

  <Organization
    title="The University of Melbourne"
    location="Melbourne, Australia"
  >
    <MelbourneUniversityLogo slot="logo" />

    <Position
      title="PhD Candidate"
      subtitle="Theoretical Particle Physics"
      start="{new Date('2018-01-01')}"
      end="{new Date('2022-06-24')}"
      keywords="{[
        'theoretical particle physics',
        'boltzmann equations',
        'matter anti-matter asymmetry',
        'leptogenesis',
        'neutrino mass',
      ]}"
    >
      <p class="unstyled text-lg md:text-xl">
        Bubbling brooks of blueberry juice cascade down the lush mountainside,
        their sweet scent mingling with the crisp, clean air. The sun sets
        slowly over the horizon, casting long shadows across the verdant
        landscape. As the world around us fades into a dreamlike haze, we are
        left with nothing but the sound of our own breathing, the rhythm of our
        hearts beating as one. It is a moment frozen in time, a memory that will
        last forever.
      </p>
    </Position>

    <Position
      title="Masters of Science"
      subtitle="Theoretical Particle Physics"
      start="{new Date('2016-01-01')}"
      end="{new Date('2017-11-30')}"
      keywords="{[
        'theoretical particle physics',
        'leptogenesis',
        'neutrino mass',
      ]}"
    >
      <p class="unstyled text-lg md:text-xl">
        A flock of rainbow-colored birds flit through the sky, their feathers
        catching the light like shards of glass. They sing a haunting melody,
        their voices echoing off the walls of the ancient ruins below. The ruins
        themselves are a testament to the power of human ingenuity, a
        labyrinthine maze of crumbling stone and twisted metal. It is a place of
        wonder and mystery, a world beyond our understanding.
      </p>
    </Position>

    <Position
      title="Bachelor of Science (Physics)"
      subtitle="Diploma in Mathematical Science"
      start="{new Date('2011-02-01')}"
      end="{new Date('2015-11-30')}"
      keywords="{[
        'theoretical particle physics',
        'leptogenesis',
        'neutrino mass',
      ]}"
    >
      <p class="unstyled text-lg md:text-xl">
        In the depths of the ocean, schools of luminous fish dart through the
        murky depths. Their scales shimmer with an otherworldly glow, their
        movements hypnotic and mesmerizing. Around them, the water pulses with a
        strange energy, a force that seems to emanate from the very heart of the
        earth. It is a world of magic and mystery, a place where anything is
        possible.
      </p>
    </Position>
  </Organization>
</section>
