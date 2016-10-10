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
  int n, a, b;
  cin >> n >> a >> b;
  string s;
  cin >> s;
  int ca = 0, cb = 0;
  REP(i, 0, n) {
    bool pass;
    switch(s[i]) {
    case 'a':
      if (ca + cb < a + b) {
	ca++;
	pass = true;
      } else {
	pass = false;
      }
      break;
    case 'b':
      if (ca + cb < a + b && cb < b) {
	cb++;
	pass = true;
      } else {
	pass = false;
      }
      break;
    default:
      pass = false;
      break;
    }
    cout << (pass ? "Yes" : "No") << endl;
  }
}
