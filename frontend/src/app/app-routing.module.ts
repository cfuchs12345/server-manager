import { Routes } from '@angular/router';
import { AuthGuard } from './auth/auth.guard';
import { RegisterGuard } from './auth/register.guard';
import { LoginComponent } from './login/login.component';
import { MainComponent } from './main/main.component';
import { RegisterComponent } from './register/register.component';

export const APP_ROUTES: Routes = [
  { path: '', redirectTo: '/login', pathMatch: 'full' },
  { path: 'login', component: LoginComponent, canActivate: [RegisterGuard] },
  { path: 'register', component: RegisterComponent },
  { path: 'home', component: MainComponent, canActivate: [AuthGuard] },
];
