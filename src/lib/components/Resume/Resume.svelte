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
<div id="resume">
  <h1>Resume</h1>

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
      <ul>
        <li>
          <ul>
            <li>
              Designed and implemented a data analytics platform from scratch,
              using Azure (including Function App and Data Factory) and
              Snowflake
            </li>
            <li>
              Integrated CI/CD workflows in Azure DevOps to automate the
              deployment of documentation and Bicep infrastructure-as-code.
            </li>
            <li>
              Implemented some examplar data pipelines as starting point for the
              client, taking Excel and API data through to the curated layer on
              Snowfkale while ensuring there is an audit trail.
            </li>
          </ul>
        </li>
        <li>
          <ul>
            <li>
              Conducted a thorough review of a client's program to update its
              integration capabilities.
            </li>
            <li>
              The review focused on three critical aspects: the business need,
              clairty and scope driving the implementation; the capabilities
              being provided by the platform; and the suitability of the
              technology design.
            </li>
            <li>
              Provided the client with insightful recommendations as to the
              program's strengths and areas for improvement.
            </li>
          </ul>
        </li>
        <li>
          <ul>
            <li>
              Technology lead and scrum aster responsible for driving the
              development of a foundational and modular Azure Data PLatform
              accelerator as well as a configuration-driven orchestrator
              framework for Data Factory.
            </li>
            <li>
              Developed a modular and extensible Python framework to facilitate
              the deployment, configuration and evelopment of the Azure Data
              Platform accelerator.
            </li>
            <li>
              The accelerator has now been successfully used in a number of
              client engagments leading to a faster delivery to the clients, a
              reduction in the cost of delivery and a higher quality of the
              solution.
            </li>
          </ul>
        </li>
        <li>
          <strong>Tech lead</strong> and <strong>scrum master</strong> on an
          internal project using <strong>Bicep infrastructure-as-code</strong>
          to facilitate the deployment of
          <strong>Azure data infrastructure</strong>. This project was
          successfully used in a client engagements both within Australia and
          overseas allowing for a much faster delivery to the clients.
        </li>
        <li>
          Pioneered the deployment of <strong
            >machine learning infrastructure</strong
          > within KPMG globally. Worked within a tightly regulated environment to
          ensure the safety of highly confidential client data.
        </li>
        <li>
          Solely responsible to the deployment and configuration of
          <strong>Azure</strong> infrastructure for a
          <strong>data analytics platform</strong>. The platform was responsible
          for ingesting data from a number of sources (from Excel to APIs), and
          processed the data through a several layers of curation ultimately
          pushing it into a Snowflake database. Implemented a sample of data
          pipelines showcasing the new platform for the client, and functioning
          as a reference for them to build their own pipelines.
        </li>
        <li>
          Piloted the use of a new <strong
            >developer standard operating environment</strong
          >, paving the way for an
          <strong>improved developer experience</strong> within KPMG Australia and
          hopefully globally. Engaged with a number of stakeholders to ensure a mutually
          satisfactory outcome that ensures a good developer experience while assuring
          KPMG's security and compliance requirements are met.
        </li>
      </ul>
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
      <ul>
        <li>
          Worked on an internal project creating a library of reusable Azure
          bicep templates to quickly deploy infrastructure, specifically
          targeting data and machine learning related services.
        </li>
      </ul>
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
      <ul>
        <li>
          Worked on an internal project creating a library of reusable Azure
          bicep templates to quickly deploy infrastructure, specifically
          targeting data and machine learning related services.
        </li>
      </ul>
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
      <ul>
        <li>
          Worked on an internal project creating a library of reusable Azure
          bicep templates to quickly deploy infrastructure, specifically
          targeting data and machine learning related services.
        </li>
      </ul>
    </Position>
  </Organization>
</div>

<style lang="postcss">
  #resume {
    @apply flex flex-col justify-center;

    h1 {
      @apply p-6 mb-12;
      @apply text-3xl md:text-5xl font-bold text-center;
      @apply shadow-xl;
      @apply bg-surface-100-800-token;
    }
  }
</style>
