#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 300100;
VI g[N];

// http://beet-aizu.hatenablog.com/entry/2017/12/12/235950
struct HLDecomposition {
  int n,pos;
  vector<vector<int> > G;
  vector<int> vid, head, sub, hvy, par, dep, inv, type;
  
  HLDecomposition(){}
  HLDecomposition(int sz):
    n(sz),pos(0),G(n),
    vid(n,-1),head(n),sub(n,1),hvy(n,-1),
    par(n),dep(n),inv(n),type(n){}
  
  void add_edge(int u, int v) {
    G[u].push_back(v);
    G[v].push_back(u);
  }

  void build(vector<int> rs=vector<int>(1,0)) {
    int c=0;
    for(int r:rs){
      dfs(r);
      bfs(r, c++);
    }
  }
  
  void dfs(int rt) {
    using T = pair<int,int>;
    vector<T> st;
    par[rt]=-1;
    dep[rt]=0;
    st.push_back(T(rt, 0));
    while(!st.empty()){
      int v=st.back().first;
      int &i=st.back().second;
      if(i<(int)G[v].size()){
	int u=G[v][i++];
	if(u==par[v]) continue;
	par[u]=v;
	dep[u]=dep[v]+1;
        st.push_back(T(u, 0));
      }else{
	st.pop_back();
	int res=0;
	for(int u:G[v]){
	  if(u==par[v]) continue;
	  sub[v]+=sub[u];
	  if(res<sub[u]) res=sub[u],hvy[v]=u;
	}
      }
    }
  }

  void bfs(int r,int c) {
    int &k=pos;
    queue<int> q({r});
    while(!q.empty()){
      int h=q.front();q.pop();
      for(int i=h;i!=-1;i=hvy[i]) {
	type[i]=c;
	vid[i]=k++;
	inv[vid[i]]=i;
	head[i]=h;
	for(int j:G[i])
	  if(j!=par[i]&&j!=hvy[i]) q.push(j);
      }
    }
  }
  
  // for_each(vertex)
  // [l,r] <- attention!!
  void for_each(int u, int v, const function<void(int, int)>& f) {
    while(1){
      if(vid[u]>vid[v]) swap(u,v);
      f(max(vid[head[v]],vid[u]),vid[v]);
      if(head[u]!=head[v]) v=par[head[v]];
      else break;
    }
  }
  
  int lca(int u,int v) const {
    while(1){
      if(vid[u]>vid[v]) swap(u,v);
      if(head[u]==head[v]) return u;
      v=par[head[v]];
    }
  }

  int distance(int u,int v){
    return dep[u]+dep[v]-2*dep[lca(u,v)];
  }
};

int num_occ[N];
int size[N];
map<int, VI> heads;
void path_val_add(int vhead, int vv, int delta) {
  if (delta == 1) {
    heads[vhead].push_back(vv - vhead);
  }
}

void process_heads(int delta) {
  for (auto &h: heads) {
    int vh = h.first;
    VI &vi = h.second;
    sort(vi.begin(), vi.end());
    if (DEBUG) {
      cerr << "heads[" << vh << "]:";
      REP(i, 0, vi.size()) cerr << " " << vi[i];
      cerr << endl;
    }
    int rem = vi.size();
    vector<PI> freq;
    int pre = 0;
    REP(i, 0, vi.size()) {
      if (i == (int) vi.size() - 1 || vi[i] != vi[i + 1]) {
        freq.push_back(PI(vi[i], i + 1 - pre));
        pre = i + 1;
      }
    }
    pre = -1;
    REP(i, 0, freq.size()) {
      int val = freq[i].first;
      int len = freq[i].second;
      num_occ[rem] += (val - pre) * delta;
      num_occ[0] -= (val - pre) * delta;
      rem -= len;
      pre = val;
    }
  }
}



VI solve(const VI &a, const HLDecomposition &hld) {
  int k = a.size();
  REP(i, 0, k) {
    int v = a[i];
    while (1) {
      int h = hld.head[v];
      path_val_add(hld.vid[h], hld.vid[v], 1);
      if (h == 0) break;
      v = hld.par[h];
    }
  }
  if (DEBUG) {
    cerr << "num_occ_prelim:";
    REP(i, 0, 10) cerr << " " << num_occ[i];
    cerr << endl;
  }
  process_heads(1);
  VI ret(k + 1);
  REP(i, 0, k + 1) {
    ret[i] = num_occ[i];
  }
  // cleanup
  process_heads(-1);
  REP(i, 0, k) {
    int v = a[i];
    while (1) {
      int h = hld.head[v];
      path_val_add(hld.vid[h], hld.vid[v], -1);
      if (h == 0) break;
      v = hld.par[h];
    }
  }
  heads.clear();
  if (DEBUG) {
    cerr << "cleanup:";
    REP(i, 0, 10) cerr << " " << num_occ[i];
    cerr << endl;
  }
  return ret;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  HLDecomposition hld(n);
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    g[u].push_back(v);
    g[v].push_back(u);
    hld.add_edge(u, v);
  }
  // HL decomposition
  hld.build();
  /*
  cerr << "vid:";
  REP(i, 0, n) cerr << " " << hld.vid[i];
  cerr << endl;
  cerr << "head:";
  REP(i, 0, n) cerr << " " << hld.head[i];
  */
  cerr << endl;
  REP(i, 0, n) {
    int v = hld.vid[i];
    int h = hld.vid[hld.head[i]];
    size[h] = max(size[h], v - h + 1);
  }
  num_occ[0] = n;
  int q;
  cin >> q;
  REP(i, 0, q) {
    int k;
    cin >> k;
    VI a(k);
    REP(j, 0, k) {
      cin >> a[j];
      a[j]--;
    }
    VI ans = solve(a, hld);
    REP(i, 0, k + 1) {
      cout << ans[i] << (i == k ? "\n" : " ");
    }
  }
}
