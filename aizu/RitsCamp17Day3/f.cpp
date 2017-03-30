#include <algorithm>
#include <iostream>
#include <cstdlib>
#include <string>
#include <set>
#include <vector>
 
 
#define rep(i, n) for (int i = 0; i < int(n); ++i)
 
using namespace std;
typedef vector<int> VI;
typedef long long ll;
const ll mod = 1e9 + 7;
const int N = 310001;
ll var[N];
 
void init_hash(void) {
  rep(i, N) {
    var[i] = rand() % mod;
  }
}
 
ll get_hash(int v, const vector<VI> &edges, int par = -1, int depth = 0) {
  ll prod = 1;
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) { continue; }
    ll s = get_hash(w, edges, v, depth + 1);
    prod = prod * ((var[depth] + s) % mod) % mod;
  }
  return prod;
}
 
void dfs(int v,  const vector<VI> &edges, VI &dist, set<int> &vis,
     int par = -1, int d = 0) {
  dist[v] = d;
  vis.insert(v);
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) {
      continue;
    }
    dfs(w, edges, dist, vis, v, d + 1);
  }
}
 
// Reuses dist
vector<int> get_centre(int v, const vector<VI> &edges, VI &dist, vector<bool> &vis) {
  set<int> vis_set;
  dfs(v, edges, dist, vis_set);
  int maxi1 = v;
  for (set<int>::iterator it = vis_set.begin(); it != vis_set.end(); ++it) {
    int i = *it;
    vis[i] = true;
    if (dist[maxi1] < dist[i]) {
      maxi1 = i;
    }
  }
  dfs(maxi1, edges, dist, vis_set);
  int maxi2 = maxi1;
  for (set<int>::iterator it = vis_set.begin(); it != vis_set.end(); ++it) {
    int i = *it;
    if (dist[maxi2] < dist[i]) {
      maxi2 = i;
    }
  }
  // max1 --- max2 is a diameter
  VI ret;
  int diam = dist[maxi2];
  if (diam % 2 == 0) {
    // The center is unique
    int cur = maxi2;
    while (dist[cur] > diam / 2) {
      rep(i, edges[cur].size()) {
	int w = edges[cur][i];
	if (dist[w] == dist[cur] - 1) {
	  cur = w;
	  break;
	}
      }
    }
    ret = VI(1, cur);
  } else {
    // Two centers
    int cur = maxi2;
    int tmp = -1;
    while (dist[cur] > diam / 2) {
      rep(i, edges[cur].size()) {
	if (dist[cur] == diam / 2 + 1) {
	  tmp = cur;
	}
	int w = edges[cur][i];
	if (dist[w] == dist[cur] - 1) {
	  cur = w;
	  break;
	}
      }
    }
    ret = VI(2);
    ret[0] = tmp;
    ret[1] = cur;
  }
  for (set<int>::iterator it = vis_set.begin(); it != vis_set.end(); ++it) {
    dist[*it] = -1; // re-use dist
  }
  return ret;
}
 
#define BEGIN_STACK_EXTEND(size) void * stack_extend_memory_ = malloc(size);void * stack_extend_origin_memory_;char * stack_extend_dummy_memory_ = (char*)alloca((1+(int)(((long long)stack_extend_memory_)&127))*16);*stack_extend_dummy_memory_ = 0;asm volatile("mov %%rsp, %%rbx\nmov %%rax, %%rsp":"=b"(stack_extend_origin_memory_):"a"((char*)stack_extend_memory_+(size)-1024));

#define END_STACK_EXTEND asm volatile("mov %%rax, %%rsp"::"a"(stack_extend_origin_memory_));free(stack_extend_memory_);


int main() {
  BEGIN_STACK_EXTEND(128*1024*1024);
  init_hash();
  int n1, m1;
  cin >> n1 >> m1;
  vector<VI> edges1(n1);
  rep(i, m1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges1[u].push_back(v);
    edges1[v].push_back(u);
  }
  int n2;
  cin >> n2;
  vector<VI> edges2(n2);
  rep(i, n2 - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges2[u].push_back(v);
    edges2[v].push_back(u);
  }
  int cnt = 0;
  vector<bool> dummy(n2, false);
  vector<int> dummy_dist(n2, -1);
  VI centre2 = get_centre(0, edges2, dummy_dist, dummy);
  vector<ll> hash2;
  rep(i, centre2.size()) {
    int c = centre2[i];
    hash2.push_back(get_hash(c, edges2));
  }
  sort(hash2.begin(), hash2.end());
  vector<bool> vis(n1, false);
  VI dist(n1, -1);
  rep(i, n1) {
    if (vis[i]) { continue; }
    VI centre1 = get_centre(i, edges1, dist, vis);
    if (centre1.size() != centre2.size()) { continue; }
    vector<ll> hash1;
    rep(i, centre1.size()) {
      int c = centre1[i];
      hash1.push_back(get_hash(c, edges1));
    }
    sort(hash1.begin(), hash1.end());
    if (hash1 == hash2) {
      cnt += 1;
    }
  }
  cout << cnt << endl;
  END_STACK_EXTEND;
}
