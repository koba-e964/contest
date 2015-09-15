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

const int N = 1001000;
int a[N];
int inv[N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
    inv[a[i]] = i;
  }
  inv[n] = -1;
  string s;
  s.resize(n, '+');
  char cur = 'A';
  s[a[0]] = 'A';
  REP(i, 1, n) {
    //  Since s[a[i - 1]..] < s[a[i]..], 
    // if s[a[i - 1] + 1 ..] < s[a[i] + 1 ..], s[a[i-1]] <= s[a[i]].
    // Otherwise, s[a[i-1]] < s[a[i]] (strictly larger)
    if (inv[a[i - 1] + 1] < inv[a[i] + 1]) {
      s[a[i]] = cur;
    } else {
      cur++;
      s[a[i]] = cur;
    }
    if (cur > 'Z') {
      cout << -1 << endl;
      return 0;
    }
  }
  cout << s << endl;
}
