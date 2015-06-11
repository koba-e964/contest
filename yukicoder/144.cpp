#include <bits/stdc++.h>
using namespace std;
const int N=1e6 + 10;
double a[N];
int main(){
  int n;
  double p;
  cin >> n >> p;
  a[0] = a[1] = 0;
  for(int i=2;i<N;i++) { 
    a[i]=1; 
  }
  for (int i = 2; i < N; i++) {
    for (int j = i * 2; j < N; j += i) {
      a[j] *= (1 - p);
    }
  }
  double s = 0;
  for (int i=2;i<=n;i++) {
    s += a[i];
  }
  printf("%.9f\n", s);
}
