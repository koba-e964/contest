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



int main(void){
  int n, m;
  cin >> n >> m;
  map<int, int> freq;
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    freq[u] += 1;
    freq[v] += 1;
  }
  for (map<int, int>::iterator it = freq.begin(); freq.end() != it; ++it) {
    if (it->second % 2 != 0) {
      cout << "NO" << endl;
      return 0;
    }
  }
  cout << "YES" << endl;
}
