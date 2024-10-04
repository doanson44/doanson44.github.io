import{j as e}from"./jsx-runtime-DEdD30eg.js";import{useMDXComponents as r}from"./index-CcnH5Kt0.js";import{ae as t}from"./index-CH_ev1HU.js";import"./index-RYns6xqu.js";import"./iframe-Dmka-Lwy.js";import"../sb-preview/runtime.js";import"./index-DAfSkmQi.js";import"./index-D-8MO0q_.js";import"./index-ar2LJKLv.js";import"./index-DrFu-skq.js";function o(i){const n={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...r(),...i.components};return e.jsxs(e.Fragment,{children:[e.jsx(t,{title:"Guidelines"}),`
`,e.jsx(n.h1,{id:"guidelines",children:"Guidelines"}),`
`,e.jsx(n.h2,{id:"requirements",children:"Requirements"}),`
`,e.jsxs(n.p,{children:["Clone all the functionalities of the input in ",e.jsx(n.a,{href:"https://just-an-input.vercel.app/?path=/story/example-input--primary",rel:"nofollow",children:"this link"})]}),`
`,e.jsx(n.h3,{id:"props",children:"Props"}),`
`,e.jsxs(n.ul,{children:[`
`,e.jsxs(n.li,{children:[e.jsx(n.code,{children:"placeholder"}),": Placeholder of the input"]}),`
`,e.jsxs(n.li,{children:[e.jsx(n.code,{children:"onSelectItem"}),": On click item handler"]}),`
`]}),`
`,e.jsx(n.h3,{id:"functions",children:"Functions"}),`
`,e.jsxs(n.ul,{children:[`
`,e.jsxs(n.li,{children:["Handle 4 state",`
`,e.jsxs(n.ul,{children:[`
`,e.jsxs(n.li,{children:[e.jsx(n.code,{children:"initital"}),": show input with placeholder"]}),`
`,e.jsxs(n.li,{children:[e.jsx(n.code,{children:"fetching"}),": fetching data, show input with a loader"]}),`
`,e.jsxs(n.li,{children:[e.jsx(n.code,{children:"success"}),': feteched data successfully, show list item or text "No results" if there are no results']}),`
`,e.jsxs(n.li,{children:[e.jsx(n.code,{children:"error"}),": fetched data failed, show error message"]}),`
`]}),`
`]}),`
`,e.jsxs(n.li,{children:["When click into an item, trigger the ",e.jsx(n.code,{children:"onSelectItem"})," function"]}),`
`,e.jsx(n.li,{children:"Add a debounce 100ms when fetching data"}),`
`,e.jsx(n.li,{children:"Handle race condition"}),`
`]}),`
`,e.jsx(n.h2,{id:"rules",children:"Rules"}),`
`,e.jsxs(n.ul,{children:[`
`,e.jsxs(n.li,{children:["Implement your functionalities inside the component ",e.jsx(n.code,{children:"src/components/Input"})," between these pair of comments:"]}),`
`]}),`
`,e.jsx(n.pre,{children:e.jsx(n.code,{children:`// Your code start here\r
  ...\r
// Your code end here
`})}),`
`,e.jsxs(n.ul,{children:[`
`,e.jsx(n.li,{children:"Do not modify any other existing components/files. Feel free to create new components if needed"}),`
`,e.jsxs(n.li,{children:["use ",e.jsx(n.code,{children:"fetchData"})," and ",e.jsx(n.code,{children:"debounce"})," functions from ",e.jsx(n.code,{children:"src/utils"})]}),`
`,e.jsxs(n.li,{children:["use ",e.jsx(n.code,{children:"Loader"})," components from ",e.jsx(n.code,{children:"src/components/Loader"}),". Feel free to create another loader if you like."]}),`
`]}),`
`,e.jsx(n.h2,{id:"how-to-get-the-code",children:"How to get the code"}),`
`,e.jsxs(n.ul,{children:[`
`,e.jsxs(n.li,{children:["Folk ",e.jsx(n.a,{href:"https://github.com/duannx/input-search-component-template.git",rel:"nofollow",children:"this repository"})]}),`
`,e.jsx(n.li,{children:"Create a new branch from the main branch. The name of the branch is your work email: E.g. vu.nguyen, nam.le"}),`
`,e.jsx(n.li,{children:"Do you work and push it to your repository"}),`
`,e.jsxs(n.li,{children:["Create a PR to the ",e.jsx(n.a,{href:"https://github.com/duannx/input-search-component-template.git",rel:"nofollow",children:"source repository"}),`.\r
The name of the PR is `,e.jsx(n.code,{children:"[Your name] - [Your work email (**without @niteco.se**)]"})]}),`
`]}),`
`,e.jsx(n.h2,{id:"how-to-deploy",children:"How to deploy"}),`
`,e.jsxs(n.ul,{children:[`
`,e.jsx(n.li,{children:"Deploy to a free online hosting like Vercel, Netlify, Cloudflare Page..."}),`
`,e.jsxs(n.li,{children:["Build command: ",e.jsx(n.code,{children:"yarn build-storybook"})]}),`
`,e.jsxs(n.li,{children:["Folder to deploy: ",e.jsx(n.code,{children:"storybook-static"})]}),`
`]}),`
`,e.jsx(n.h3,{id:"dont-forget-to-submit-your-works-by-filling-the-information-on-the-excel-file",children:"Don't forget to submit your works by filling the information on the Excel file"})]})}function p(i={}){const{wrapper:n}={...r(),...i.components};return n?e.jsx(n,{...i,children:e.jsx(o,{...i})}):o(i)}export{p as default};
