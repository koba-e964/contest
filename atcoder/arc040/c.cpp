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

const int N = 101;
string s[N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> s[i];
  }
  int c = 0;
  REP(i, 0, n) {
    int t = n-1;
    while (t >= 0 && s[i][t] == 'o') {
      --t;
    }
    if (t != -1) {
      c++;
      if (i < n -1) {
	REP(j, t, n) {
	  s[i + 1][j] = 'o';
	}
      }
    }
  }
  cout << c << endl;
}
