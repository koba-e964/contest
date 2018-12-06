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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

void output(bool winning) {
  cout << (!winning ? "Second" : "First") << endl;
  exit(0);
}

int main(void){
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  sort(a.rbegin(), a.rend());
  a.push_back(0);
  int k = 0;
  while (k < n && k < a[k]) k++;
  // Where is the intersection point of y=x and block's border?
  int start = k;
  while (start >= 0) {
    if (a[k] == a[start]) start--;
    else break;
  }
  start++;
  int end = k;
  while (end < n) {
    if (a[end] == a[k]) end++;
    else break;
  }
  if (k == a[k]) {
    if (start == k) {
      output((end - k) % 2 == 1 || (a[k - 1] - k) % 2 == 1); 
    }
    output((end - k) % 2 == 1);
  }
  output((a[k - 1] - k) % 2 == 1);
}
