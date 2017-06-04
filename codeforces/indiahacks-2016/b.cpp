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


set<string> vis;
set<string> sol;

int n, q;
const int N = 36;
string a[N];
char b[N];

void dfs(const string &s) {
  if ((int) s.length() == n) {
    sol.insert(s);
    return;
  }
  if (vis.count(s)) {
    return;
  }
  vis.insert(s);
  REP(j, 0, q) {
    if (b[j] != s[0]) { continue; }
    string tmp = a[j] + s.substr(1);
    dfs(tmp);
  }
}


int main(void){
  cin >> n >> q;
  REP(i, 0, q) {
    cin >> a[i] >> b[i];
  }
  dfs("a");
  cout << sol.size() << endl;
}
