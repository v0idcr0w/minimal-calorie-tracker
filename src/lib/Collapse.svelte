<script>
    import * as Collapsible from '$lib/components/ui/collapsible';
    import {ChevronsUpDown} from 'lucide-svelte';
    export let title; 
</script>

<Collapsible.Root>
    <div class="flex items-center justify-between space-x-10">
        <h4 class="text-[15px] font-medium">{title}</h4>
        <Collapsible.Trigger
          class="inline-flex h-10 w-10 items-center justify-center rounded-9px border border-border-input bg-background-alt text-foreground shadow-btn transition-all hover:bg-muted active:scale-98"
        >
          <ChevronsUpDown class="size-4 " weight="bold" />
          <span class="sr-only">Toggle</span>
        </Collapsible.Trigger>
    </div>
    <Collapsible.Content>
    <slot/>
    </Collapsible.Content>
  </Collapsible.Root>

