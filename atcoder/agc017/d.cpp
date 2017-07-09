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

const int N = 123456;
VI edges[N];


int dfs(int v, int par) {
  int sum = 0;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) { continue; }
    int res = dfs(w, v);
    sum ^= res + 1;
  }
  return sum;
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    edges[x].push_back(y);
    edges[y].push_back(x);
  }
  cout << (dfs(0, -1) == 0 ? "Bob" : "Alice") << endl;
}
