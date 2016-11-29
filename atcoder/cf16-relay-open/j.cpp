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
  int n;
  cin >> n;
  vector<PI> pool;
  REP(i, 0, n - 1) {
    if (i % 2 == 1) {
      pool.push_back(PI(0, i));
      pool.push_back(PI(i, 0));
    }
  }
  REP(i, 0, n) {
    if ((i + n - 1) % 2 == 1) {
      pool.push_back(PI(n - 1, i));
      pool.push_back(PI(i, n - 1));
    }
  }
  for (int j = 1; j < n - 1; j += 3) {
    REP(i, 1, n - 1) {
      if ((i + j) % 2 == 1) {
	pool.push_back(PI(i, j));
      }
    }
  }
  cout << pool.size() << endl;
  REP(i, 0, pool.size()) {
    cout << pool[i].first << " " << pool[i].second << endl;
  }
}
