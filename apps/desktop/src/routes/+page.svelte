<script lang="ts">
  import TopNavbar from '$lib/components/navbar/TopNavbar.svelte';
  import EndpointList from '$lib/components/sidebar/EndpointList.svelte';
  import RequestBuilder from '$lib/components/request/RequestBuilder.svelte';
  import ResponseViewer from '$lib/components/response/ResponseViewer.svelte';
  import { Separator } from "$lib/components/ui/separator/index.js";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
  import { Splitpanes, Pane } from 'svelte-splitpanes';
  import GlobalErrorOverlay from '$lib/components/GlobalErrorOverlay.svelte';
</script>

<svelte:head>
  <title>Reqsmith Desktop</title>
</svelte:head>

<div class="h-screen bg-background text-foreground flex flex-col">
  <!-- Top Navbar -->
  <TopNavbar />
  
  <!-- Main Content -->
  <div class="flex-1 overflow-hidden">
    <Splitpanes class="default-theme splitpanes-shadcn" horizontal={false}>
      <!-- Sidebar -->
      <Pane size={22} minSize={16} maxSize={40} class="bg-background">
        <div class="p-3 h-full flex flex-col">
          <ScrollArea class="flex-1 bg-background">
            <EndpointList />
          </ScrollArea>
        </div>
      </Pane>

      <!-- Builder -->
      <Pane size={46} minSize={30}>
        <div class="h-full overflow-hidden p-4 bg-background">
          <RequestBuilder />
        </div>
      </Pane>

      <!-- Response -->
      <Pane size={32} minSize={24}>
        <div class="h-full overflow-hidden p-4 bg-background">
          <ResponseViewer />
        </div>
      </Pane>
    </Splitpanes>
  </div>
  
  <!-- Keep the app alive and show any runtime error -->
  <GlobalErrorOverlay />
</div>
