#include <stdio.h>
#include <string.h>
#include <vector>
#include <bitset>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int len;
char s[8001];
int x,y;

vector<int> a,b;

bool ok(const vector<int> &a,int x){
  if(x<-8000||x>8000)return 0;
  typedef bitset<16001> bs;
  bs cur;
  cur[8000]=true;
  for(int b:a){
    if(b==0)continue;
    bs tmp=cur<<b|cur>>b;
    cur=tmp;
  }
  return cur[8000+x];
}

int main(void){
  scanf("%8001s",s);
  len=strlen(s);
  scanf("%d%d",&x,&y);
  int cur=0;
  int dir=0;
  REP(i,0,len+1){
    if(i<len&&s[i]=='F')cur++;
    else {
      if(dir==0)x-=cur;
      else if(dir==1)b.push_back(cur);
      else a.push_back(cur);
      dir=dir==1?2:1;
      cur=0;
    }
  }
  puts(ok(a,x)&&ok(b,y)?"Yes":"No");
}
