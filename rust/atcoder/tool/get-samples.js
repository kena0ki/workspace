const puppeteer = require('puppeteer');
const fs = require('fs');

const contestId=process.argv[2] || 261;
const sampleDir=process.argv[3] || `samples`;

(async () => {
  console.log("logging in to the website");
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('https://atcoder.jp/login');
  await page.type('#username', process.env.AC_USER);
  await page.type('#password', process.env.AC_PASSWORD);
  await page.click('#submit');
  //await page.waitForNavigation();
  console.log("logged in");
  console.log(process.argv);
  await page.goto(`https://atcoder.jp/contests/${contestId}/tasks`);
  const links = await page.evaluate(() => {
    return Array.from(document.querySelectorAll('tr td:first-child a')).map(v=>v.href);
  });
  console.log(links);
  for (const link of links) {
    await page.goto(link);
    const sampleTexts = await page.evaluate(() => {
      return Array.from(document.querySelectorAll('[id^=pre-sample]'))
        .filter((_,i,{length}) =>(i<length/2)).map(v=>v.textContent);
    });
    const problemId = link[link.length-1].toLowerCase();
    fs.rmSync(`${sampleDir}/${problemId}`,{recursive:true,force:true});
    fs.mkdirSync(`${sampleDir}/${problemId}`,{recursive:true});
    for (let i=0; i<sampleTexts.length/2; i++) {
      fs.writeFileSync(`${sampleDir}/${problemId}/in${i+1}`, sampleTexts[2*i]);
      fs.writeFileSync(`${sampleDir}/${problemId}/out${i+1}`, sampleTexts[2*i+1]);
    }
  }
  console.log('Done');

  await browser.close();
})();
