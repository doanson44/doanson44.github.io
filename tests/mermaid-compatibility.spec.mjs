import { test, expect } from '@playwright/test';

const BASE_URL = 'http://127.0.0.1:8080';

const diagrams = [
  ['flowchart', 'flowchart LR\n  A[Start] --> B[End]'],
  ['swimlane', 'swimlane-beta LR\n  subgraph Customer\n    request[Request]\n  end\n  subgraph Support\n    answer[Answer]\n  end\n  request --> answer'],
  ['sequence', 'sequenceDiagram\n  Alice->>Bob: Hello\n  Bob-->>Alice: Hi'],
  ['class', 'classDiagram\n  class User\n  User : +String name\n  User --> Account'],
  ['state', 'stateDiagram-v2\n  [*] --> Idle\n  Idle --> Active\n  Active --> [*]'],
  ['er', 'erDiagram\n  CUSTOMER ||--o{ ORDER : places\n  ORDER ||--|{ LINE_ITEM : contains'],
  ['user-journey', 'journey\n  title User Journey\n  section Visit\n    Open site: 5: User\n    Read page: 4: User'],
  ['gantt', 'gantt\n  title Project\n  dateFormat YYYY-MM-DD\n  section Work\n    Task :done, 2026-08-01, 3d'],
  ['pie', 'pie title Browser usage\n  "Chrome" : 60\n  "Other" : 40'],
  ['quadrant', 'quadrantChart\n  title Reach and engagement\n  x-axis Low --> High\n  y-axis Low --> High\n  quadrant-1 Expand\n  quadrant-2 Promote\n  quadrant-3 Re-evaluate\n  quadrant-4 Improve\n  Campaign A: [0.3, 0.6]'],
  ['requirement', 'requirementDiagram\n  requirement req {\n    id: 1\n    text: The system shall render diagrams\n    risk: low\n    verifymethod: test\n  }'],
  ['gitgraph', 'gitGraph\n  commit\n  branch develop\n  checkout develop\n  commit\n  checkout main\n  merge develop'],
  ['c4', 'C4Context\n  title System Context\n  Person(user, "User")\n  System(system, "Markdown Studio")\n  Rel(user, system, "Uses")'],
  ['mindmap', 'mindmap\n  root((Markdown))\n    Editor\n    Preview\n    Mermaid'],
  ['timeline', 'timeline\n  title Releases\n  2025 : Version 1\n  2026 : Version 2'],
  ['zenuml', 'zenuml\n  title Demo\n  Alice->John: Hello\n  John->Alice: Hi'],
  ['sankey', 'sankey-beta\n  Source,Target A,100\n  Source,Target B,50\n  Target A,Final,100'],
  ['xychart', 'xychart-beta\n  title "Sales"\n  x-axis [Jan, Feb, Mar]\n  y-axis "Revenue" 0 --> 100\n  bar [20, 50, 80]\n  line [10, 60, 90]'],
  ['block', 'block-beta\n  columns 3\n  client["Client"] api["API"] db[("Database")]\n  client --> api\n  api --> db'],
  ['packet', 'packet-beta\n  title TCP Packet\n  0-15: "Source Port"\n  16-31: "Destination Port"\n  32-63: "Sequence Number"'],
  ['kanban', 'kanban\n  todo[Todo]\n    task[Implement test]\n  done[Done]\n    finished[Review]'],
  ['architecture', 'architecture-beta\n  group api(cloud)[API]\n  service db(database)[Database] in api\n  service server(server)[Server] in api\n  db:L -- R:server'],
  ['radar', 'radar-beta\n  axis A, B, C, D, E\n  curve one{1,2,3,4,5}\n  curve two{5,4,3,2,1}'],
  ['event-modeling', 'eventmodeling\n  tf 01 ui CartUI\n  tf 02 cmd AddItem\n  tf 03 evt ItemAdded'],
  ['treemap', 'treemap-beta\n  "Products"\n    "Tools": 10\n    "Games": 5'],
  ['venn', 'venn-beta\n  title "Team overlap"\n  set Frontend\n  set Backend\n  union Frontend,Backend["APIs"]'],
  ['ishikawa', 'ishikawa-beta\n  User problem\n    Process\n      Slow workflow\n    Equipment\n      Old device'],
  ['wardley', 'wardley-beta\n  title Simple Map\n  Business -> Product\n  Product -> Platform\n  evolve Platform 0.8'],
  ['cynefin', 'cynefin-beta\n  title Complexity\n  complex\n  "Emergent work"\n  clear\n  "Known procedure"'],
  ['treeview', 'treeView-beta\n  project/\n    src/\n      main.rs\n    README.md'],
];

const markdownSource = diagrams
  .map(([name, code]) => ['## ' + name, '', '```mermaid', code, '```', ''].join('\n'))
  .join('\n');

test.describe('Markdown Mermaid compatibility', () => {
  test('renders every supported Mermaid diagram type', async ({ page }) => {
    await page.goto(`${BASE_URL}/#/tools/markdown`);
    await page.locator('#markdown-editor').waitFor();

    const results = await page.evaluate(async (items) => {
      const output = [];
      for (const [name, code] of items) {
        const response = await window.__mermaid_render(`compat-${name}`, code);
        output.push([name, JSON.parse(response)]);
      }
      return output;
    }, diagrams);

    for (const [name, response] of results) {
      expect(response.ok, `${name}: ${response.error ?? 'render failed'}`).toBe(true);
      expect(response.svg, `${name}: missing SVG`).toContain('<svg');
    }
  });

  test('renders every Mermaid type through the Markdown Studio preview', async ({ page }) => {
    await page.goto(`${BASE_URL}/#/tools/markdown`);
    const editor = page.locator('#markdown-editor');
    await editor.waitFor();
    await editor.fill(markdownSource);

    await expect(page.locator('#markdown-preview-content .mermaid-container')).toHaveCount(diagrams.length, {
      timeout: 15000,
    });
    await expect(page.locator('#markdown-preview-content .mermaid-error')).toHaveCount(0, {
      timeout: 30000,
    });
    await expect(page.locator('#markdown-preview-content .mermaid-diagram svg')).toHaveCount(diagrams.length, {
      timeout: 30000,
    });
  });
});
