#include <algorithm>
#include <bitset>
#include <cstdlib>
#include <cstring>
#include <cstdio>
#include <cassert>
#include <iostream>
#include <sstream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

ll a,b,c,pos,read;
const int N=1<<23;
ll s[N],t[N],u[N];
void solve(){
  assert (pos <= N);
  if (pos == 0) return;
  ll inf=6e9;
  ll lo=-1,hi=inf;
  for(;hi-lo>1;){
    ll mid=(lo+hi)/2;
    ll cnt=0;
    REP(i,0,pos){
      ll intm=s[i]<=min(t[i],mid)?(min(t[i],mid)-s[i])/u[i]+1:0;
      cnt+=intm;
    }
    if(cnt%2)
      hi=mid;
    else
      lo=mid;
  }
  if(hi==inf)
    cout << "no corruption\n";
  else{
    read=0;
    REP(i,0,pos){
      if(s[i]<=hi&&hi<=t[i]&&(hi-s[i])%u[i]==0)read++;
    }
    assert (read%2==1);
    cout << hi << " " << read << "\n";
  }
}
char buf[1025];
bool init(){
  int f = 0;
  pos = 0;
  while((f = ((fgets(buf, 1024, stdin)!=NULL))) && strlen(buf) > 2){
    sscanf(buf, "%llu%llu%llu", s+pos, t+pos, u+pos);
    pos++;
  }
  return f || pos > 0;
}
int main(void){
  // IO consulted by http://d.hatena.ne.jp/komiyam/20120207/1328545633
  string st;
  while(init()){
    solve();
  }

}
