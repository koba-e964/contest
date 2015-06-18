#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

int b[101] = {};
int a[101];

int loop_size(int v) {
  if (a[v] == -1) {
    return 1;
  }
  int q = a[v];
  a[v] = -1;
  int c = 1;
  while (q != v) {
    int e = a[q];
    a[q] = -1;
    q = e;
    c++;
  }
  return c;
}

int main(void){
  int n;
  cin >> n;
  int c = 0;
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  REP(i, 0, n) {
    b[loop_size(i)]++;
  }
  REP(i, 0, 50) {
    if (b[2 * i] % 2) {
      cout << "No" << endl;
      return 0;
    }
  }
  cout << "Yes" << endl;
}
