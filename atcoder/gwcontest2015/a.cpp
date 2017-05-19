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
  int sc[10] = {25,39,51,76,163,111,136,128,133,138};
  set<int> cand;
  REP(bits, 0, 1024) {
    int tot = 0;
    REP(i, 0, 10) {
      if (bits & 1 << i) {
	tot += sc[i];
      }
    }
    cand.insert(tot);
    if (bits & 1 << 6) {
      cand.insert(tot - 78);
    }
  }
  for (set<int>::iterator it = cand.begin(); it != cand.end(); ++it) {
    cout << *it << endl;
  }
  
}
