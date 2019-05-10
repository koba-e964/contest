#include <algorithm>
#include <cstdio>
#include <set>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;

const int N=100010;
int u[N];
int freq[N];
int freq2[N];
set<int> app;

int main(void) {
  int n;
  scanf("%d",&n);
  REP(i,0,n)scanf("%d",u+i);
  int ma=0;
  REP(i,0,n){
    int v=u[i];
    if(freq2[freq[v]]==1)app.erase(freq[v]);
    freq2[freq[v]]--;
    freq[v]++;
    freq2[freq[v]]++;
    if(freq2[freq[v]]==1)app.insert(freq[v]);
    if(app.size()>2)continue;
    if(app.size()==1){
      int v=*app.begin();
      if(freq2[v]==1||v==1)ma=i+1;
    }
    if(app.size()==2){
      auto it=app.begin();
      int fst=*it++;
      int snd=*it;
      if((fst==1&&freq2[fst]==1)||(snd==fst+1&&freq2[snd]==1)){
        ma=i+1;
      }
    }
  }
  printf("%d\n",ma);
}
