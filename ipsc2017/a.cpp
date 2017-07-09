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
  int t;
  cin >> t;
  while (t--) {
    int r, c;
    cin >> r >> c;
    vector<PI> pool;
    for (int i = 0; i < r; i += 2) {
      pool.push_back(PI(i, 0));
    }
    for (int i = 2; i < c; i += 2) {
      pool.push_back(PI(0, i));
    }
    if (r % 2 == 0 || c % 2 == 0) {
      pool.push_back(PI(r - 1, c - 1));
    }
    cout << pool.size() << endl;
    REP(i, 0, pool.size()) {
      cout << pool[i].first << " " << pool[i].second << endl;
    }
  }
}
