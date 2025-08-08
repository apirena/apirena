<script lang="ts">
  import ProjectSelector from '$lib/components/sidebar/ProjectSelector.svelte';
  import EndpointList from '$lib/components/sidebar/EndpointList.svelte';
  import RequestBuilder from '$lib/components/request/RequestBuilder.svelte';
  import ResponseViewer from '$lib/components/response/ResponseViewer.svelte';
  import { Button } from "$lib/components/ui/button/index.js";
  import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
  import { Splitpanes, Pane } from 'svelte-splitpanes';
</script>

<svelte:head>
  <title>Reqsmith Desktop</title>
</svelte:head>

<div class="h-screen bg-background text-foreground">
  <Splitpanes class="default-theme splitpanes-shadcn" horizontal={false}>
    <!-- Sidebar -->
    <Pane size={22} minSize={16} maxSize={40} class="bg-background">
      <div class="p-3 h-full flex flex-col">
        <Card class="shadow-sm">
          <CardHeader class="pb-2">
            <CardTitle class="text-base font-semibold flex items-center gap-2">
              <span class="text-lg">🔧</span>
              Reqsmith
            </CardTitle>
          </CardHeader>
          <CardContent class="pt-0 text-xs text-muted-foreground">
            API Playground
            <div class="mt-3 flex gap-2">
              <Button size="sm">New Request</Button>
            </div>
          </CardContent>
        </Card>

        <Separator class="my-3" />

        <div class="space-y-3">
          <ProjectSelector />
        </div>

        <Separator class="my-3" />

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
