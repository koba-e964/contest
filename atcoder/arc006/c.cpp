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
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
  int n;
  cin >> n;
  VI w(n);
  REP(i, 0, n) {
    cin >> w[i];
  }
  VI mnt;
  REP(i, 0, n) {
    sort(mnt.begin(), mnt.end());
    bool done = 0;
    REP(m, 0, mnt.size()) {
      if (!done && w[i] <= mnt[m]) {
	mnt[m] = w[i];
	done = 1;
	break;
      }
    }
    if (!done) {
      mnt.push_back(w[i]);
    }
  }
  cout << mnt.size() << endl;
}
