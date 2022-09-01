<!-- This file is generated via `cargo build`. Please don't edit by hand. -->

<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-emphasis -->
<!-- cSpell:disable -->

# Modifiers

<div class="admonition summary">
<p class="admonition-title">Grammar</p>

<pre style="white-space: pre-wrap;"><code id="ModifierDefinitionProduction"><span style="color: var(--md-code-hl-keyword-color);">ModifierDefinition</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">"modifier"</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#IdentifierProduction">«Identifier»</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/08-functions#ParameterListProduction">ParameterList</a></span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">{</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/09-modifiers#ModifierAttributeProduction">ModifierAttribute</a></span><span style="color: var(--md-code-hl-operator-color);">}</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">(</span><span style="color: var(--md-code-hl-string-color);">';'</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../04-statements/01-blocks#BlockProduction">Block</a></span><span style="color: var(--md-code-hl-operator-color);">)</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="ModifierAttributeProduction"><span style="color: var(--md-code-hl-keyword-color);">ModifierAttribute</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/08-functions#OverrideSpecifierProduction">OverrideSpecifier</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"virtual"</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="ModifierInvocationProduction"><span style="color: var(--md-code-hl-keyword-color);">ModifierInvocation</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#IdentifierPathProduction">IdentifierPath</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/08-functions#ArgumentListProduction">ArgumentList</a></span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

</div>

--8<-- "specification/notes/02-definitions/09-modifiers/index.md"