#include <algorithm>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int n;
const int N = 200010;
int a[N];

int solve0() {
  int s = 0, t = n - 1;
  int cnt = 0;
  while (s < t) {
    if (a[s] + a[t] == 0) {
      cnt ++;
      s++;
      t--;
      continue;
    }
    if (a[s] + a[t] > 0) {
      t--;
      continue;
    }
    s++;
  }
  return cnt;
}

int solve_pos() {
  int s = 0, t = n - 1;
  int cnt = 0;
  while (s < t) {
    if (a[s] + a[t] > 0) {
      cnt++;
      s++;
      t--;
      continue;
    }
    s++;
  }
  return cnt;
}

int main(void){
  cin >> n;
  n *= 2;
  REP(i, 0, n) {
    cin >> a[i];
  }
  sort(a, a + n);
  int a0 = solve0();
  int a1 = solve_pos();
  REP(i, 0, n) {
    a[i] = -a[i];
  }
  reverse(a, a + n);
  int a2 = solve_pos();
  cout << a2 << " " << a1 << " " << a0 << endl;
}
