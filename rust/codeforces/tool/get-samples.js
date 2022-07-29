const puppeteer = require('puppeteer');
const fs = require('fs');

const contestId=process.argv[2] || 1710;
const sampleDir=process.argv[3] || `samples`;

(async () => {
  console.log("logging in to the website");
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('https://codeforces.com/enter');
  await page.type('#handleOrEmail', process.env.CF_EMAIL);
  await page.type('#password', process.env.CF_PASSWORD);
  await page.click('input.submit');
  await page.waitForNavigation();
  console.log("logged in");
  console.log(process.argv);
  await page.goto(`https://codeforces.com/contest/${contestId}`);
  const links = await page.evaluate(() => {
    return Array.from(document.querySelectorAll('td.id a')).map(v=>v.href);
  });
  console.log(links);
  for (const link of links) {
    await page.goto(link);
    const inputs = await page.evaluate(() => {
      return Array.from(document.querySelectorAll(".input pre")).map(v=>v.textContent);
    });
    const outputs = await page.evaluate(() => {
      return Array.from(document.querySelectorAll(".output pre")).map(v=>v.textContent);
    });
    const l = link.split('/');
    const problemId = l[l.length-1].toLowerCase();
    fs.rmSync(`${sampleDir}/${problemId}`,{recursive:true,force:true});
    fs.mkdirSync(`${sampleDir}/${problemId}`,{recursive:true});
    for (let i=0; i<inputs.length; i++) {
      fs.writeFileSync(`${sampleDir}/${problemId}/in${i+1}`, inputs[i]);
      fs.writeFileSync(`${sampleDir}/${problemId}/out${i+1}`, outputs[i]);
    }
  }
  console.log(`Done`);

  await browser.close();
})();
