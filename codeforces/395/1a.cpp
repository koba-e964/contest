#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 100100;
VI edges[N];
int col[N];

int ec[N] = {};

void dfs(int v, vector<PI> &ch, int p = -1) {
  for (auto w: edges[v]) {
    if (w == p) { continue; }
    if (col[v] != col[w]) {
      ch.push_back(PI(v, w));
      ec[v]++;
      ec[w]++;
    }
    dfs(w, ch, v);
  }
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  REP(i, 0, n) {
    cin >> col[i];
  }
  vector<PI> ch;
  dfs(0, ch);
  int cons = -1;
  REP(i, 0, n) {
    if (ec[i] == ch.size()) {
      cons = i;
    }
  }
  if (cons == -1) {
    cout << "NO" << endl;
    return 0;
  }
  cout << "YES" << endl;
  cout << cons + 1 << endl;
}
