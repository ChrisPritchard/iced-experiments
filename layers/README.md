# Layers

A project containing a new widget type that allows positioning elements in arbitrary places, overlapping.

Sort of a planned distillation of the modal and toast example projects into something more easily usable.

Idea will be something like this:

Normal, non overlapping:

    Root Element
        Element
            Element
            Element
        Element
        Element

New Layer Element Type:

    Root Element
        Element
        Layer Element (top_left, size)
            Element
            Element
            Element
        Layer Element
            Element
            Element
            Element
        Element

In the above, i see the second layer overlapping the first, but both overlapping elements attached to the root.