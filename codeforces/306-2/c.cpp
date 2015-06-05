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
  string s;
  cin >> s;
  s = string("00") + s;
  int n = s.length();
  REP(i,0,n) {
    REP(j,i+1,n) {
      REP(k,j+1,n) {
	int t = (s[i] - '0') * 100 + (s[j] - '0') * 10 + s[k] - '0';
	if (t % 8 == 0) {
	  cout << "YES" << endl;
	  cout << t << endl;
	  return 0;
	}
      }
    }
  }
  cout << "NO" << endl;
}
