<!--
Resume component, used to display a resume in a nice way and makes use of the
Section component to display each section of the resume.
-->
<script lang="ts">
  import Organization from "./Organization.svelte";
  import Position from "./Position.svelte";
  import KPMGLogo from "./assets/kpmg.svg?component";
  import MelbourneUniversityLogo from "./assets/melbourne_university.svg?component";
  import SmartBearLogo from "./assets/smartbear.svg?component";
</script>

<!-- Resume -->
<section id="resume" class="flex flex-col justify-center">
  <h1
    class="p-6 mb-12 text-3xl md:text-5xl font-bold text-center shadow-xl bg-surface-100-800-token"
  >
    Resume
  </h1>

  <Organization title="PactFlow - SmartBear" location="Melbourne, Australia">
    <SmartBearLogo slot="logo" />

    <Position
      title="Senior Software Engineer"
      subtitle="Python, Rust and Open Source"
      start="{new Date('2023-09-04')}"
      end="{null}"
      keywords="{[
        'open source',
        'pact',
        'pactflow',
        'python',
        'rust',
        'software engineering',
      ]}"
    >
      <div class="flex flex-col space-y-4">
        <p>
          I am spearheading a critical <strong>upgrade of Pact Python</strong>
          to support Pact the latest Pact specifications. This involves a significant
          rewrite of the core library to make use of the foreign function interface
          (FFI) provided by the Pact core Rust library. This will allow uses of Pact
          Python to reap the full benefit of the new features provided in the latest
          Pact specifications.
        </p>
        <p>
          My responsibilities extend beyond coding to include active engagement
          with the Pact developer community. I am offering assistance to
          open-source contributors, taking part in community meetups
        </p>
      </div>
    </Position>
  </Organization>

  <Organization title="KPMG" location="Melbourne, Australia">
    <KPMGLogo slot="logo" />

    <Position
      title="Senior Consultant"
      subtitle="Data & DevOps Engineer"
      start="{new Date('2022-06-27')}"
      end="{new Date('2023-08-25')}"
      keywords="{[
        'azure',
        'consulting',
        'data engineering',
        'devops',
        'infrastructure-as-code',
        'python',
      ]}"
    >
      <div class="flex flex-col space-y-4">
        <p>
          I have successfully delivered multiple complex projects in various
          industries. These projects involved:
        </p>
        <ul class="list-outside list-disc pl-8 flex flex-col space-y-4">
          <li>
            Deploying <strong>machine learning infrastructure</strong> within a
            tightly regulated environment, processing
            <strong>highly confidential</strong> information, while working closely
            with stakeholders across cloud infrastructure, security, and the data
            science teams to ensure a good outcome. This project has also been recognized
            for pioneering the deployment of machine learning infrastructure within
            KPMG globally.
          </li>
          <li>
            Implementing a <strong>data analytics platform</strong> from
            scratch, using Azure and Snowflake, and integrating
            <strong>CI/CD workflows</strong> in Azure DevOps to automate deployment
            of documentation and Bicep infrastructure-as-code. This project included
            implementing two model data pipelines as starting points for the client,
            taking Excel and API data through to a curated layer.
          </li>
          <li>
            Conducting a <strong>thorough review</strong> of a university's Integration
            Capabilities Uplift program's platform, with a focus on the business
            need, clarity and scope driving the implementation, the capabilities
            being provided by the platform, and the suitability of the technology
            design. Our team provided the client with insightful recommendations
            as to the program's strengths and areas of improvement.
          </li>
          <li>
            Acting as the <strong>Technology Lead</strong> and
            <strong>Scrum Master</strong> for a foundational and modular
            <strong>Azure Data Platform accelerator</strong> and a
            <strong>configuration-driven orchestrator</strong>
            framework for Data Factory, building upon the former accelerator. I also
            developed a modular and extensible Python framework to facilitate the
            deployment, configuration, and development of the Azure Data Platform
            accelerator.
          </li>
        </ul>
        <p>
          Overall, I have a strong track record of <strong
            >delivering innovative and effective technology solutions</strong
          > while collaborating closely with stakeholders to ensure successful outcomes.
        </p>
      </div>
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
      start="{new Date('2017-01-01')}"
      end="{new Date('2022-06-24')}"
      keywords="{[
        'theoretical particle physics',
        'boltzmann equations',
        'matter anti-matter asymmetry',
        'leptogenesis',
        'neutrino mass',
      ]}"
    >
      <div class="flex flex-col space-y-4">
        <p>
          Conducted research on an <strong
            >E6-inspired seesaw neutrino mass</strong
          >
          model under the guidance of Prof. Volkas, exploring the relevant parameter
          space through the
          <strong>generation and visualization of large data sets</strong>.
        </p>
        <p>
          Developed a new computational tool to solve <strong
            >highly coupled Boltzmann differential equations</strong
          >
          involving hundreds of terms and rates spanning multiple orders of magnitude.
          This tool allows for the calculation of the evolution of particle densities
          and asymmetries in the early universe, and the resulting matter--anti-matter
          asymmetry. This was developed in
          <strong>Rust</strong> and featured novel integration quadratures to
          efficiently evaluate certain integrals numerically, and required
          extensive analysis and optimization of
          <strong>numerical algorithms</strong> to ensure results were both accurate
          and computable within a reasonable timeframe.
        </p>
        <p>
          Utilized the above-mentioned computational tool to investigate the
          validity of common simplifying assumptions and explore previously
          difficult-to-reach parameter space in two-Higgs doublet models.
        </p>
      </div>
    </Position>

    <Position
      title="Masters of Science"
      subtitle="Theoretical Particle Physics"
      start="{new Date('2015-02-01')}"
      end="{new Date('2016-12-30')}"
      keywords="{[
        'theoretical particle physics',
        'leptogenesis',
        'neutrino mass',
      ]}"
    >
      <div class="flex flex-col space-y-4">
        <p>
          Investigated the phenomenology of an E<sub>6</sub>-inspired seesaw
          neutrino mass model under the guidance of Prof. Volkas. Explored the
          relevant parameter space through the generation and visualization of
          large data sets and constrained the parameter space using experimental
          data from the LHC and cosmological observations.
        </p>
        <p>
          Completed coursework achieving a 1<sup>st</sup> class honours average.
        </p>
        <p>
          In my spare time, created <strong>TikZ-Feynman</strong>, a
          <strong>LaTeX</strong> package for drawing Feynman diagrams. This has become
          widely adopted by the particle physics community, with over 400 citations.
        </p>
      </div>
    </Position>

    <Position
      title="Bachelor of Science (Physics)"
      subtitle="Diploma in Mathematical Science"
      start="{new Date('2011-02-01')}"
      end="{new Date('2014-11-30')}"
      keywords="{[
        'theoretical particle physics',
        'leptogenesis',
        'neutrino mass',
      ]}"
    >
      <div class="flex flex-col space-y-4">
        <p>
          Completed a <strong>major in physics</strong> and concurrent
          <strong>diploma in mathematical science</strong>
          with a focus on pure mathematics. Completed coursework achieving a 1<sup
            >st</sup
          >
          class honours average in both and was a recipient of the
          <strong>Dean's List Award</strong> for my performance.
        </p>
        <p>
          Attended the <strong>CERN summer school</strong> in 2013 where I
          helped developed a <strong>level 1 trigger for the LHC</strong> in C++
          and ROOT.
        </p>
      </div>
    </Position>
  </Organization>
</section>
